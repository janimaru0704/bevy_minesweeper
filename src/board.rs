use bevy::{color::palettes::tailwind, prelude::*};

use crate::constants;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_board);
    }
}

// タイルのコンポーネント. 左下からの座標を持つ
#[derive(Component)]
#[allow(dead_code)]
struct Tile {
    x: u32,
    y: u32,
}

// 盤面を敷く
fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 起点となる左下の座標
    let offset_x = -constants::BOARD_W / 2.0 + (constants::TILE_SIZE / 2.0);
    let offset_y = -constants::BOARD_H / 2.0 + (constants::TILE_SIZE / 2.0);

    for x in 0..constants::TILE_COLUMNS {
        for y in 0..constants::TILE_ROWS {
            commands.spawn((
                Tile { x, y },
                Mesh2d(meshes.add(Rectangle::from_length(constants::TILE_SIZE - 1.0))),
                MeshMaterial2d(
                    materials.add(ColorMaterial::from_color(Color::from(tailwind::GRAY_400))),
                ),
                Transform::from_xyz(
                    offset_x + (x as f32 * constants::TILE_SIZE),
                    offset_y + (y as f32 * constants::TILE_SIZE),
                    0.0,
                ),
            ));
        }
    }
}
