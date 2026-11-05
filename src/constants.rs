//! 定数を補完するモジュール

/// ウィンドウのサイズ(横)
pub const WINDOW_W: f32 = 800.0;
/// ウィンドウのサイズ(縦)
pub const WINDOW_H: f32 = 880.0;

// UI
/// 画面上部UI部分のサイズ(横)
pub const UI_W: f32 = WINDOW_W;
/// 画面上部UI部分のサイズ(縦)
pub const UI_H: f32 = 80.0;
/// UIの左右の余白
pub const UI_PADDING_H: f32 = 60.0;
/// タイマーと残り地雷カウンタのフォントサイズ
pub const COUNTER_FONT_SIZE: f32 = 40.0;
/// リセットボタンの余白(横)
pub const RESET_BUTTON_PADDING_HORIZONTAL: f32 = 10.0;
/// リセットボタンの余白(縦)
pub const RESET_BUTTON_PADDING_VERTICAL: f32 = 5.0;
/// リセットボタンのフォントサイズ
pub const RESET_BUTTON_FONT_SIZE: f32 = 36.0;

// 盤面とタイル
/// 盤面表示部分のサイズ(横)
pub const BOARD_W: f32 = WINDOW_W;
/// 盤面表示部分のサイズ(縦)
pub const BOARD_H: f32 = WINDOW_H - UI_H;
/// タイルのサイズ(縦横共通)
pub const TILE_SIZE: f32 = 40.0;
/// タイルの横方向の数
pub const TILE_COLUMNS: usize = (BOARD_W / TILE_SIZE) as usize;
/// タイルの縦方向の数
pub const TILE_ROWS: usize = (BOARD_H / TILE_SIZE) as usize;

/// 地雷の数
pub const MINE_COUNT: usize = 60;
