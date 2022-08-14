pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_WIDTH: usize = 400;
pub const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

pub const VIEWPORT_HEIGHT: f64 = 2.0;
pub const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
pub const FOCAL_LENGTH: f64 = 1.0;