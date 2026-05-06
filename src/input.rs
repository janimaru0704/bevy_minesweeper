use bevy::{prelude::*, window::PrimaryWindow};

use crate::constants;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_input);
    }
}

// 右クリックか左クリックかの内容
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ClickButton {
    Left,
    Right,
}

// マウスの入力をboardに伝えるイベント
// タイルのインデックスと右or左クリックかの情報を持つ
#[derive(Event)]
pub struct TileClickEvent {
    pub x: u32,
    pub y: u32,
    pub button: ClickButton,
}

fn mouse_input(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let is_left = mouse_input.just_pressed(MouseButton::Left);
    let is_right = mouse_input.just_pressed(MouseButton::Right);
    if !is_left && !is_right {
        return;
    }

    // マウスのスクリーン座標を取得
    if let Ok(window) = windows.single()
        && let Ok((camera, camera_transform)) = camera_query.single()
        && let Some(cursor_pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    {
        // タイルのインデックスに変換
        let x = ((cursor_pos.x + constants::BOARD_W / 2.0) / constants::TILE_SIZE).floor() as u32;
        let y = ((cursor_pos.y + constants::BOARD_H / 2.0) / constants::TILE_SIZE).floor() as u32;

        // 範囲内かチェック
        if x < constants::TILE_COLUMNS && y < constants::TILE_ROWS {
            commands.trigger(TileClickEvent {
                x,
                y,
                button: if is_left {
                    ClickButton::Left
                } else {
                    ClickButton::Right
                },
            });
        }
    }
}
