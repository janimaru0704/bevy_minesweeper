use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (load_font, spawn_node).chain());
    }
}

// 画面ノードのマーカー
#[derive(Component)]
struct UINode;

// フォントのリソース
#[derive(Resource)]
struct FontHandle(Handle<Font>);

// フォントの読み込み
fn load_font(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.insert_resource(FontHandle(assets.load("NotoSansJP-VariableFont_wght.ttf")));
}

// UIノードの配置
fn spawn_node(mut commands: Commands, font: Res<FontHandle>) {
    commands.spawn((
        UINode,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
    )).with_children(|p| {
        // テスト用のテキストを描画
        p.spawn((
            Text::new("TEST TEXT"),
            TextFont {
                font: font.0.clone(),
                weight: FontWeight::BOLD,
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    });

}