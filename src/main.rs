use bevy::{prelude::*, window::WindowResolution, winit::WinitSettings};

mod constants;
mod ui;

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
        .add_plugins(ui::UIPlugin)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .run();
}

// 初期化
fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    // 背景色を黒色に
    clear_color.0 = Color::BLACK;

    // カメラを生成. 上40pxにUIを表示するのでずらす
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, constants::UI_H / 2.0, 0.0),
    ));
}
