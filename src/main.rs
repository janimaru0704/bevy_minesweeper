use bevy::{prelude::*, window::WindowResolution, winit::WinitSettings};

mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(720, 750),
                title: "MINESWEEPER".into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .run();
}

// 初期化
fn setup(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>
) {
    commands.spawn(Camera2d);
    clear_color.0 = Color::BLACK;
}