// ウィンドウサイズ
pub const WINDOW_W: f32 = 800.0;
pub const WINDOW_H: f32 = 880.0;

// UI
pub const UI_W: f32 = WINDOW_W;
pub const UI_H: f32 = 80.0;
pub const UI_PADDING: f32 = 60.0;

// タイルとボード
pub const BOARD_W: f32 = WINDOW_W;
pub const BOARD_H: f32 = WINDOW_H - UI_H;

pub const TILE_SIZE: f32 = 40.0;
pub const TILE_COLUMNS: usize = (BOARD_W / TILE_SIZE) as usize;
pub const TILE_ROWS: usize = (BOARD_H / TILE_SIZE) as usize;

// 地雷の数
pub const MINE_COUNT: usize = 60;
