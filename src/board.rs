use bevy::{color::palettes::tailwind, prelude::*};
use rand::seq::SliceRandom;

use crate::{constants, input, ui};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_board)
            .add_systems(Update, update_board_visual)
            .add_observer(process_input)
            .add_observer(reset_board_state);
    }
}

// タイルのコンポーネント. 左下からの座標を持つ
#[derive(Component)]
#[allow(dead_code)]
struct Tile {
    x: usize,
    y: usize,
}

// タイル用テキストのマーカー
#[derive(Component)]
struct TileText;

// タイルの種類
#[derive(Clone, Copy, PartialEq, Debug)]
enum TileType {
    Empty(u8),
    Mine,
}

impl Default for TileType {
    fn default() -> Self {
        Self::Empty(0)
    }
}

// タイルの見た目 (隠れている/開いている/旗が立っている)
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum TileAppearance {
    #[default]
    Hidden,
    Revealed,
    Flagged,
}

// 1つのタイルの状態を格納する構造体
#[derive(Clone, Copy, PartialEq, Debug, Default)]
struct TileState {
    tile_type: TileType,
    appearance: TileAppearance,
}

// 盤面の状態を管理するリソース
#[derive(Resource)]
struct Board(Vec<TileState>);

// ボードリセットを要求するイベント
#[derive(Event)]
struct ResetBoardEvent;

impl Board {
    fn get_index(x: usize, y: usize) -> usize {
        y * constants::TILE_COLUMNS + x
    }

    // タイルの情報を取得
    fn get_tile(&self, x: usize, y: usize) -> &TileState {
        &self.0[Self::get_index(x, y)]
    }

    // タイルの情報を取得(可変参照)
    fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut TileState {
        &mut self.0[Self::get_index(x, y)]
    }

    // Revealedでないなら旗をトグルする
    fn toggle_flag(&mut self, x: usize, y: usize) {
        let tile = self.get_tile_mut(x, y);
        tile.appearance = match tile.appearance {
            TileAppearance::Hidden => TileAppearance::Flagged,
            TileAppearance::Flagged => TileAppearance::Hidden,
            TileAppearance::Revealed => TileAppearance::Revealed,
        };
    }

    // 連鎖して開く
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
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        // 範囲内かチェック
                        if 0 <= nx
                            && nx < constants::TILE_COLUMNS as i32
                            && 0 <= ny
                            && ny < constants::TILE_ROWS as i32
                        {
                            stack.push((nx as usize, ny as usize));
                        }
                    }
                }
            }
        }
    }
}

// 盤面を敷く
fn spawn_board(mut commands: Commands, font: Res<ui::FontHandle>) {
    // 盤面の初期化
    commands.trigger(ResetBoardEvent);

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
                            font: font.noto_sans.clone(),
                            font_size: 32.0,
                            weight: FontWeight::BOLD,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                    ));
                });
        }
    }
}

// ボードの見た目の更新
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
                                5 => Color::from(tailwind::RED_500),
                                6 => Color::from(tailwind::TEAL_400),
                                7 => Color::from(tailwind::PURPLE_500),
                                8 => Color::from(tailwind::ZINC_500),
                                _ => text_color.0,
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

// 入力を受け付けて、ボードを更新
fn process_input(event: On<input::TileClickEvent>, mut board: ResMut<Board>) {
    // ボタンによって分岐
    match event.button {
        input::ClickButton::Left => {
            // タイルを開き、開けたなら次へ
            if board.get_tile(event.x, event.y).tile_type == TileType::Mine {
                // TODO ゲームオーバー処理
            } else {
                board.open_chain(event.x, event.y);
            }
        }
        input::ClickButton::Right => {
            // 開いていないなら、旗をトグルする
            board.toggle_flag(event.x, event.y);
        }
    }
}

// 盤面をリセット
fn reset_board_state(_: On<ResetBoardEvent>, mut commands: Commands) {
    let mut board = Board(vec![
        TileState::default();
        constants::TILE_COLUMNS * constants::TILE_ROWS
    ]);

    // 地雷を配置
    place_mines(&mut board);

    // 地雷数を計算
    calc_mine_numbers(&mut board);

    // 盤面を上書き
    commands.insert_resource(board);
}

// 地雷をランダムに配置
fn place_mines(board: &mut Board) {
    let mut rng = rand::rng();
    let total_tiles = constants::TILE_COLUMNS * constants::TILE_ROWS;

    // 全インデックスのリストを作成し、シャッフルする
    let mut indices: Vec<usize> = (0..total_tiles).collect();
    indices.shuffle(&mut rng);

    // シャッフルされた先頭から個数分取り出してそのインデックスに地雷を置く
    for &idx in indices.iter().take(constants::MINE_COUNT) {
        board.0[idx].tile_type = TileType::Mine;
    }
}

// 地雷数を計算して、Emptyマスの値に格納
fn calc_mine_numbers(board: &mut Board) {
    for y in 0..constants::TILE_ROWS {
        for x in 0..constants::TILE_COLUMNS {
            // 地雷マスはスキップ
            if board.get_tile(x, y).tile_type == TileType::Mine {
                continue;
            }

            // 周囲の数を確認
            let mut count = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    // 自分自身はスキップ
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    // 確認先の座標を取得
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    // 範囲内であるか確認
                    if 0 <= nx
                        && nx < constants::TILE_COLUMNS as i32
                        && 0 <= ny
                        && ny < constants::TILE_ROWS as i32
                        && board.get_tile(nx as usize, ny as usize).tile_type == TileType::Mine
                    {
                        count += 1;
                    }
                }
            }
            // 計算した値をセット
            board.get_tile_mut(x, y).tile_type = TileType::Empty(count);
        }
    }
}
