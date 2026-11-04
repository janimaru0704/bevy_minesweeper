use bevy::{prelude::*, window::WindowResolution};

mod board;
mod constants;
mod input;
mod ui;

// ゲームの状態を管理する列挙型
#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Debug, Default)]
pub enum GameState {
    // ゲーム開始前。タイマーは停止，爆弾設置前。最初のマスが開かれたらInGameに移動
    #[default]
    PreGame,

    // ゲーム中。タイマーカウント。爆弾のないマスを全て開けたらGameClearに，爆弾を踏んだらGameOverに移動
    InGame,

    // 爆弾のない全てのマスをプレイヤーが開いた後のクリア画面。タイマーは動かず，リセットボタン以外の操作を受け付けない。リセットボタンを押したらPreGameへ
    GameClear,

    // 爆弾を踏んでゲームオーバーになった状態。爆弾のマスが全て表示され，安全なのに旗が立っているマスは強調表示(未完成)。タイマー以下はGameClear画面と同じ
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(
                    constants::WINDOW_W as u32,
                    constants::WINDOW_H as u32,
                ),
                title: "MINESWEEPER".into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins((ui::UIPlugin, board::BoardPlugin, input::InputPlugin))
        .add_systems(Startup, setup)
        .run();
}

// 初期化
fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    // 背景色を黒色に
    clear_color.0 = Color::BLACK;

    // カメラを生成. 上部にUIを表示するのでずらす
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, constants::UI_H / 2.0, 0.0),
    ));
}
