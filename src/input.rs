//! 入力処理を担うモジュール

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{GameState, constants};

/// 入力処理を担うプラグイン
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_input);
    }
}

/// クリックの種類
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ClickButton {
    /// 左クリック
    Left,
    /// 右クリック
    Right,
    /// 中クリック
    Middle,
}

/// マウスの入力をboardに伝えるイベント
#[derive(Event, Debug)]
pub struct TileClickEvent {
    /// クリックされたタイルのx座標
    pub x: usize,
    /// クリックされたタイルのy座標
    pub y: usize,
    /// クリックの種類
    pub button: ClickButton,
}

/// マウス入力を管理
fn mouse_input(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    game_state: Res<State<GameState>>,
) {
    let is_left = mouse_input.just_pressed(MouseButton::Left);
    let is_right = mouse_input.just_pressed(MouseButton::Right);
    let is_middle = mouse_input.just_pressed(MouseButton::Middle);
    if !is_left && !is_right && !is_middle {
        return;
    }

    // マウスのスクリーン座標を取得
    if let Ok(window) = windows.single()
        && let Ok((camera, camera_transform)) = camera_query.single()
        && let Some(cursor_pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    {
        // タイルに対する入力
        // GameStateがPreGameかInGameの時のみ実行する
        if matches!(*game_state.get(), GameState::PreGame | GameState::InGame) {
            // タイルのインデックスに変換
            let x = ((cursor_pos.x + constants::BOARD_W / 2.0) / constants::TILE_SIZE).floor() as i32;
            let y = ((cursor_pos.y + constants::BOARD_H / 2.0) / constants::TILE_SIZE).floor() as i32;

            // 範囲内かチェック
            if (0..constants::TILE_COLUMNS as i32).contains(&x)
                && (0..constants::TILE_ROWS as i32).contains(&y)
            {
                // タイルクリックのイベントを発火
                commands.trigger(TileClickEvent {
                    x: x as usize,
                    y: y as usize,
                    button: if is_left {
                        ClickButton::Left
                    } else if is_right {
                        ClickButton::Right
                    } else {
                        ClickButton::Middle
                    },
                });
            }
        }
    }
}
