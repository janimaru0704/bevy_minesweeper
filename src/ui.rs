use crate::{GameState, board, constants};
use bevy::ui_widgets::{Activate, Button, observe};
use bevy::{color::palettes::tailwind, prelude::*};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (load_font, spawn_node).chain())
            .add_systems(Update, (update_mine_counter, update_stopwatch))
            .add_systems(OnEnter(GameState::GameOver), handle_game_over)
            .add_systems(OnEnter(GameState::GameClear), handle_game_clear);
    }
}

// 画面ノードのマーカー
#[derive(Component)]
struct UINode;

// ストップウォッチのマーカー
#[derive(Component)]
struct StopwatchText;

// 残り地雷数表示のマーカー
#[derive(Component)]
struct MineCounterText;

// リセットボタンとそのテキストのマーカー
#[derive(Component)]
struct ResetButton;
#[derive(Component)]
struct ResetButtonText;

// フォントのリソース
#[derive(Resource)]
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
    let counter_font = TextFont {
        font: FontSource::Handle(font.dseg7.clone()),
        font_size: FontSize::Px(constants::COUNTER_FONT_SIZE),
        ..default()
    };
    let reset_button_font = TextFont {
        font: FontSource::Handle(font.noto_sans.clone()),
        font_size: FontSize::Px(constants::RESET_BUTTON_FONT_SIZE),
        weight: FontWeight::BOLD,
        ..default()
    };
    commands.spawn(
        (
            UINode,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                width: Val::Px(constants::UI_W),
                height: Val::Px(constants::UI_H),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(constants::UI_PADDING_H)),
                ..default()
            },
            children![
                (
                    StopwatchText,
                    Text::new("000"),
                    counter_font.clone(),
                    TextColor(Color::from(tailwind::RED_600)),
                ),
                (
                    Node {
                        position_type: PositionType::Relative,
                        border: UiRect::all(Val::Px(3.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::axes(
                            Val::Px(constants::RESET_BUTTON_PADDING_HORIZONTAL),
                            Val::Px(constants::RESET_BUTTON_PADDING_VERTICAL)
                        ),
                        ..default()
                    },
                    Button,
                    ResetButton,
                    BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                    BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                    observe(
                        |_: On<Activate>,
                         mut next_game_state: ResMut<NextState<GameState>>,
                         mut text_query: Query<
                            (&mut Text, &mut TextColor),
                            With<ResetButtonText>,
                        >| {
                            // ゲーム状態をGameState::PreGameに遷移
                            next_game_state.set(GameState::PreGame);

                            // ボタンのテキストを元に戻す
                            if let Ok((mut text, mut color)) = text_query.single_mut() {
                                text.0 = "RESET".to_string();
                                color.0 = Color::from(tailwind::YELLOW_400);
                            }
                        }
                    ),
                    children![(
                        ResetButtonText,
                        Text::new("RESET"),
                        reset_button_font.clone(),
                        TextColor(Color::from(tailwind::YELLOW_400)),
                    )]
                ),
                (
                    MineCounterText,
                    Text::new(format!("{:03}", constants::MINE_COUNT)),
                    counter_font.clone(),
                    TextColor(Color::from(tailwind::RED_600)),
                )
            ],
        ),
    );
}

// 地雷カウンターの更新
fn update_mine_counter(
    board: Res<board::Board>,
    mut text: Query<&mut Text, With<MineCounterText>>,
) {
    // ボードに更新がないならスキップ
    if !board.is_changed() {
        return;
    }

    // 盤面の旗の数を数える
    let flags = board
        .tiles
        .iter()
        .filter(|t| t.appearance == board::TileAppearance::Flagged)
        .count();

    // テキストを更新
    if let Ok(mut text) = text.single_mut() {
        let remaining = constants::MINE_COUNT as i32 - flags as i32;
        text.0 = format!("{:03}", remaining);
    }
}

// ストップウォッチの更新
fn update_stopwatch(
    game_state: Res<board::GameResources>,
    mut text: Query<&mut Text, With<StopwatchText>>,
) {
    // GameStateに更新がないならスキップ
    if !game_state.is_changed() {
        return;
    }

    // テキストを更新
    if let Ok(mut text) = text.single_mut() {
        text.0 = format!("{:03}", game_state.time as u32);
    }
}

fn handle_game_over(
    mut query: Query<(&mut Text, &mut TextColor), With<ResetButtonText>>,
    mut board: ResMut<board::Board>,
) {
    // テキストを更新
    if let Ok((mut text, mut color)) = query.single_mut() {
        text.0 = "GAME OVER".to_string();
        color.0 = Color::from(tailwind::RED_600);
    }

    // 爆弾のマスを全て開く
    board.reveal_all_mines();
}

fn handle_game_clear(mut query: Query<(&mut Text, &mut TextColor), With<ResetButtonText>>) {
    // テキスト更新
    if let Ok((mut text, mut color)) = query.single_mut() {
        text.0 = "CLEAR".to_string();
        color.0 = Color::from(tailwind::GREEN_600);
    }
}
