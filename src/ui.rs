use bevy::{color::palettes::tailwind, prelude::*};

use crate::constants;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (load_font, spawn_node).chain());
    }
}

// 画面ノードのマーカー
#[derive(Component)]
struct UINode;

// 時間経過表示のマーカー
#[derive(Component)]
struct TimerText;

// 残り地雷数表示のマーカー
#[derive(Component)]
struct MineCountText;

// フォントのリソース
#[derive(Resource)]
#[allow(dead_code)]
pub struct FontHandle {
    pub noto_sans: Handle<Font>,
    pub dseg7: Handle<Font>,
}

// フォントの読み込み
fn load_font(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(FontHandle {
        noto_sans: assets.load("NotoSansJP-VariableFont_wght.ttf"),
        dseg7: assets.load("DSEG7Modern-Bold.ttf"),
    });
}

// UIノードの配置
fn spawn_node(mut commands: Commands, font: Res<FontHandle>) {
    commands
        .spawn((
            UINode,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                width: Val::Px(constants::UI_W),
                height: Val::Px(constants::UI_H),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(constants::UI_PADDING)),
                ..default()
            },
        ))
        .with_children(|p| {
            let text_font = TextFont {
                font: font.dseg7.clone(),
                font_size: 40.0,
                ..default()
            };

            // 経過時間表示
            p.spawn((
                TimerText,
                Text::new("000"),
                text_font.clone(),
                TextColor(Color::from(tailwind::RED_600)),
            ));

            // 残り地雷数表示
            p.spawn((
                MineCountText,
                Text::new("050"),
                text_font.clone(),
                TextColor(Color::from(tailwind::RED_600)),
            ));
        });
}
