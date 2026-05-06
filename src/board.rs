use bevy::{color::palettes::tailwind, prelude::*};

use crate::{constants, input, ui};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_board)
            .add_systems(Update, update_board)
            .add_observer(process_input);
    }
}

// タイルのコンポーネント. 左下からの座標を持つ
#[derive(Component)]
#[allow(dead_code)]
struct Tile {
    x: u32,
    y: u32,
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

// タイルの見た目 (隠れている/開いている/旗が立っている)
#[derive(Clone, Copy, PartialEq, Debug)]
enum TileAppearance {
    Hidden,
    Revealed,
    Flagged,
}

// 1つのタイルの状態を格納する構造体
#[derive(Clone, Copy, PartialEq, Debug)]
struct TileState {
    tile_type: TileType,
    appearance: TileAppearance,
}

// 盤面の状態を管理するリソース
#[derive(Resource)]
struct BoardState(Vec<TileState>);

impl BoardState {
    fn get_index(x: u32, y: u32) -> usize {
        (y * constants::TILE_COLUMNS + x) as usize
    }

    fn get_tile(&self, x: u32, y: u32) -> &TileState {
        &self.0[Self::get_index(x, y)]
    }
}

// 盤面を敷く
fn spawn_board(mut commands: Commands, font: Res<ui::FontHandle>) {
    // 盤面の初期化
    let tiles = vec![
        TileState {
            tile_type: TileType::Empty(0),
            appearance: TileAppearance::Hidden,
        };
        (constants::TILE_COLUMNS * constants::TILE_ROWS) as usize
    ];
    commands.insert_resource(BoardState(tiles));

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

// ボードの更新
fn update_board(
    board: Res<BoardState>,
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
                    text_color.0 = Color::from(tailwind::ROSE_600);
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
                                1 => Color::from(tailwind::BLUE_600),
                                2 => Color::from(tailwind::GREEN_800),
                                3 => Color::from(tailwind::RED_500),
                                4 => Color::from(tailwind::INDIGO_800),
                                5 => Color::from(tailwind::RED_800),
                                6 => Color::from(tailwind::TEAL_400),
                                7 => Color::from(tailwind::PURPLE_500),
                                8 => Color::from(tailwind::ZINC_700),
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
fn process_input(event: On<input::TileClickEvent>) {
    println!("{}, {}: {:?}", event.x, event.y, event.button);
}
