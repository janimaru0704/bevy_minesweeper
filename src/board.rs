//! 盤面のロジックと描画処理のモジュール

use bevy::{color::palettes::tailwind, prelude::*};
use rand::seq::SliceRandom;
use std::collections::HashSet;

use crate::{GameState, constants, input, ui};

/// ゲーム盤面に関する処理を担うプラグイン
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_board)
            .add_systems(
                OnEnter(GameState::PreGame),
                (reset_game_resources, reset_board),
            )
            .add_systems(
                Update,
                (
                    update_board_visual,
                    update_stopwatch.run_if(in_state(GameState::InGame)),
                ),
            )
            .add_observer(process_tile_clicked);
    }
}

/// タイルのコンポーネント。左下からの座標を持つ
#[derive(Component)]
struct Tile {
    x: usize,
    y: usize,
}

/// タイル用テキストのマーカー
#[derive(Component)]
struct TileText;

/// タイルの種類
#[derive(Clone, Copy, PartialEq, Debug)]
enum TileType {
    /// 空のタイル。数字は周囲の地雷の数
    Empty(u8),
    /// 地雷のタイル
    Mine,
}

impl Default for TileType {
    fn default() -> Self {
        Self::Empty(0)
    }
}

/// タイルの見た目
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum TileAppearance {
    /// 隠れているタイル
    #[default]
    Hidden,
    /// 開かれている
    Revealed,
    /// 旗が立てられているタイル
    Flagged,
}

/// 1つのタイルの状態を格納する構造体
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct TileState {
    /// タイルの中身
    tile_type: TileType,
    /// タイルの見た目
    pub appearance: TileAppearance,
}

/// 盤面の状態を管理するリソース
#[derive(Resource)]
pub struct Board {
    /// タイルを1次元配列で格納。
    pub tiles: Vec<TileState>,
}

impl Board {
    fn empty() -> Self {
        Board {
            tiles: vec![TileState::default(); constants::TILE_COLUMNS * constants::TILE_ROWS],
        }
    }
}

/// ストップウォッチやその他ゲーム情報を管理するリソース
#[derive(Resource, Default)]
pub struct GameResources {
    /// ストップウォッチの時間
    pub time: f32,
}

impl Board {
    /// 座標からtilesのindexに変換
    fn get_index(x: usize, y: usize) -> usize {
        y * constants::TILE_COLUMNS + x
    }

    /// (x, y)のマスの周囲8マスの座標を返すイテレータ
    fn neighbors(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        (-1i32..=1)
            .flat_map(move |dy| (-1i32..=1).map(move |dx| (dx, dy)))
            .filter(|&(dx, dy)| (dx, dy) != (0, 0))
            .filter_map(move |(dx, dy)| {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                ((0..constants::TILE_COLUMNS as i32).contains(&nx)
                    && (0..constants::TILE_ROWS as i32).contains(&ny))
                .then_some((nx as usize, ny as usize))
            })
    }

    /// タイルの情報を不変参照で取得
    /// # Panics
    /// 範囲外の座標を指定するとパニックします。
    fn get_tile(&self, x: usize, y: usize) -> &TileState {
        &self.tiles[Self::get_index(x, y)]
    }

    /// タイルの情報を可変参照で取得
    /// # Panics
    /// 範囲外の座標を指定するとパニックします。
    fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut TileState {
        &mut self.tiles[Self::get_index(x, y)]
    }

    /// タイルがRevealedでないなら旗をトグルする
    fn toggle_flag(&mut self, x: usize, y: usize) {
        let tile = self.get_tile_mut(x, y);
        tile.appearance = match tile.appearance {
            TileAppearance::Hidden => TileAppearance::Flagged,
            TileAppearance::Flagged => TileAppearance::Hidden,
            TileAppearance::Revealed => TileAppearance::Revealed,
        };
    }

    /// 与えられた座標から空のタイルを連鎖して開く
    fn open_chain(&mut self, start_x: usize, start_y: usize) {
        // 確認すべき座標を格納
        let mut stack = vec![(start_x, start_y)];

        while let Some((x, y)) = stack.pop() {
            let tile = self.get_tile_mut(x, y);

            // すでに開いている、もしくは旗が立っているならスキップ
            if tile.appearance != TileAppearance::Hidden {
                continue;
            }

            // タイルを開く
            tile.appearance = TileAppearance::Revealed;

            // このタイルが周囲に地雷がない空白なら周囲をスタックに追加
            if tile.tile_type == TileType::Empty(0) {
                stack.extend(Self::neighbors(x, y));
            }
        }
    }

    /// ミドルクリックの処理
    /// クリックされた空のマスの数字と，周囲の旗の数が一致している時に
    /// 周囲の旗の立てられていないマスを開きます。
    fn handle_middle_click(&mut self, x: usize, y: usize) -> bool {
        let tile = self.get_tile(x, y);

        // 開いている数字タイル以外は何もしない
        if tile.appearance != TileAppearance::Revealed {
            return false;
        }
        let TileType::Empty(target_num) = tile.tile_type else {
            return false;
        };
        if target_num == 0 {
            return false;
        }

        // 周囲の旗の数が数字と一致している時だけ処理する
        let flag_count = Self::neighbors(x, y)
            .filter(|&(nx, ny)| self.get_tile(nx, ny).appearance == TileAppearance::Flagged)
            .count();
        if flag_count != target_num as usize {
            return false;
        }

        // 周囲の隠れているマスを集めてから開く
        let hidden = Self::neighbors(x, y)
            .filter(|&(nx, ny)| self.get_tile(nx, ny).appearance == TileAppearance::Hidden)
            .collect::<Vec<_>>();

        for (nx, ny) in hidden {
            if self.get_tile(nx, ny).tile_type == TileType::Mine {
                // 旗の位置が間違っていた。ゲームオーバーなのでtrueを返して抜ける
                return true;
            }
            self.open_chain(nx, ny);
        }

        false
    }

    /// 地雷を指定された座標周辺を避けてランダムに配置
    fn place_mines(&mut self, click_x: usize, click_y: usize) {
        let mut rng = rand::rng();

        // クリックされた周囲3x3マスは候補から除く
        let remove_set: HashSet<_> = Self::neighbors(click_x, click_y)
            .chain(std::iter::once((click_x, click_y)))
            .collect();

        // クリックされた周囲3x3マスを除く全ての座標のリストを作成
        let mut candidates: Vec<_> = (0..constants::TILE_ROWS)
            .flat_map(move |y| (0..constants::TILE_COLUMNS).map(move |x| (x, y)))
            .filter(|pos| !remove_set.contains(pos))
            .collect();

        // シャッフル
        candidates.shuffle(&mut rng);

        // シャッフルされた先頭から個数分取り出してそのインデックスに地雷を置く
        for &(x, y) in candidates.iter().take(constants::MINE_COUNT) {
            self.get_tile_mut(x, y).tile_type = TileType::Mine;
        }

        // 各マスの周囲の地雷数を計算
        self.calc_mine_numbers();
    }

    /// 各タイルの周囲の地雷数を計算して、Emptyマスの値に格納
    fn calc_mine_numbers(&mut self) {
        for y in 0..constants::TILE_ROWS {
            for x in 0..constants::TILE_COLUMNS {
                // 地雷マスはスキップ
                if self.get_tile(x, y).tile_type == TileType::Mine {
                    continue;
                }

                // 周囲の数を確認
                let count = Self::neighbors(x, y)
                    .filter(|&(nx, ny)| self.get_tile(nx, ny).tile_type == TileType::Mine)
                    .count();

                // 計算した値をセット
                self.get_tile_mut(x, y).tile_type = TileType::Empty(count as u8);
            }
        }
    }

    /// 盤面上の地雷を全て開く
    pub fn reveal_all_mines(&mut self) {
        self.tiles
            .iter_mut()
            .filter(|tile| tile.tile_type == TileType::Mine)
            .for_each(|tile| tile.appearance = TileAppearance::Revealed);
    }

    /// 全てのEmptyマスが開かれたか(ゲームクリアか)を確認
    fn is_cleared(&self) -> bool {
        self.tiles
            .iter()
            .filter(|tile| tile.appearance == TileAppearance::Revealed)
            .count()
            == self.tiles.len() - constants::MINE_COUNT
    }
}

/// 盤面を表示するスプライトを生成
fn spawn_board(mut commands: Commands, font: Res<ui::FontHandle>) {
    // 起点となる左下の座標
    let offset_x = -constants::BOARD_W / 2.0 + (constants::TILE_SIZE / 2.0);
    let offset_y = -constants::BOARD_H / 2.0 + (constants::TILE_SIZE / 2.0);

    // タイルをすべてスポーン
    for x in 0..constants::TILE_COLUMNS {
        for y in 0..constants::TILE_ROWS {
            commands
                .spawn((
                    Tile { x, y },
                    Sprite::from_color(
                        Color::from(tailwind::GRAY_400),
                        Vec2::new(constants::TILE_SIZE - 2.0, constants::TILE_SIZE - 2.0),
                    ),
                    Transform::from_xyz(
                        offset_x + (x as f32 * constants::TILE_SIZE),
                        offset_y + (y as f32 * constants::TILE_SIZE),
                        0.0,
                    ),
                ))
                .with_children(|p| {
                    // タイルそれぞれのテキスト
                    p.spawn((
                        TileText,
                        Text2d::new(""),
                        TextFont {
                            font: FontSource::Handle(font.noto_sans.clone()),
                            font_size: FontSize::Px(32.0),
                            weight: FontWeight::BOLD,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                    ));
                });
        }
    }
}

/// ボードのスプライトの更新
fn update_board_visual(
    board: Res<Board>,
    mut tile_query: Query<(&Tile, &mut Sprite, &Children)>,
    mut text_query: Query<(&mut Text2d, &mut TextColor), With<TileText>>,
) {
    // 更新がないならスキップ
    if !board.is_changed() {
        return;
    }

    for (tile, mut sprite, children) in &mut tile_query {
        let tile_state = board.get_tile(tile.x, tile.y);

        // 背景色の更新
        sprite.color = match tile_state.appearance {
            TileAppearance::Hidden | TileAppearance::Flagged => Color::from(tailwind::GRAY_400),
            TileAppearance::Revealed => Color::from(tailwind::GRAY_600),
        };

        if let Some(&child) = children.first()
            && let Ok((mut text, mut text_color)) = text_query.get_mut(child)
        {
            // 文字と文字色の更新
            text.0 = match tile_state.appearance {
                TileAppearance::Hidden => "".to_string(),
                TileAppearance::Flagged => {
                    text_color.0 = Color::from(tailwind::ROSE_700);
                    "F".to_string()
                }
                TileAppearance::Revealed => match tile_state.tile_type {
                    TileType::Mine => {
                        text_color.0 = Color::BLACK;
                        "B".to_string()
                    }
                    TileType::Empty(num) => {
                        if num > 0 {
                            text_color.0 = match num {
                                1 => Color::from(tailwind::BLUE_500),
                                2 => Color::from(tailwind::GREEN_500),
                                3 => Color::from(tailwind::RED_500),
                                4 => Color::from(tailwind::INDIGO_500),
                                5 => Color::from(tailwind::PINK_400),
                                6 => Color::from(tailwind::TEAL_400),
                                7 => Color::from(tailwind::PURPLE_500),
                                8 => Color::from(tailwind::ZINC_500),
                                _ => unreachable!(),
                            };
                            num.to_string()
                        } else {
                            "".to_string()
                        }
                    }
                },
            };
        }
    }
}

/// 入力を受け付けて、盤面を更新
fn process_tile_clicked(
    event: On<input::TileClickEvent>,
    mut board: ResMut<Board>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    // GameStateによって分岐
    match *game_state.get() {
        GameState::PreGame => {
            // 左クリックで盤面を生成
            if event.button == input::ClickButton::Left {
                // 地雷を配置
                board.place_mines(event.x, event.y);

                // 最初のマスから連鎖的にマスを開く(地雷でないことは，配置によって保証済み)
                board.open_chain(event.x, event.y);

                // 状態をGameState::InGameに遷移
                next_game_state.set(GameState::InGame);

                // 万一全てのEmptyマスが開いた時に備え，ゲームクリアのチェック
                if board.is_cleared() {
                    next_game_state.set(GameState::GameClear);
                }
            }
        }
        GameState::InGame => {
            // ボタンによって分岐
            match event.button {
                input::ClickButton::Left => {
                    // 旗が立っているならスキップ
                    if board.get_tile(event.x, event.y).appearance == TileAppearance::Flagged {
                        return;
                    }
                    // タイルが地雷ならゲームオーバーに，地雷でないなら連鎖的に開く
                    if board.get_tile(event.x, event.y).tile_type == TileType::Mine {
                        // ゲームオーバー状態に遷移
                        next_game_state.set(GameState::GameOver);
                    } else {
                        board.open_chain(event.x, event.y);
                        if board.is_cleared() {
                            next_game_state.set(GameState::GameClear);
                        }
                    }
                }
                input::ClickButton::Right => {
                    // 旗をトグルする
                    board.toggle_flag(event.x, event.y);
                }
                input::ClickButton::Middle => {
                    if board.handle_middle_click(event.x, event.y) {
                        next_game_state.set(GameState::GameOver);
                    } else if board.is_cleared() {
                        next_game_state.set(GameState::GameClear);
                    }
                }
            }
        }
        _ => {}
    }
}

/// 盤面をリセット
fn reset_board(mut commands: Commands) {
    // 盤面をリセット
    commands.insert_resource(Board::empty());
}

/// ゲームリソースをリセット
fn reset_game_resources(mut commands: Commands) {
    commands.insert_resource(GameResources::default());
}

/// ストップウォッチを更新
fn update_stopwatch(mut game_resources: ResMut<GameResources>, time: Res<Time>) {
    game_resources.time += time.delta_secs();
}
