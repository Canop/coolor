
/// coolor error type
#[derive(thiserror::Error, Debug)]
pub enum CoolorError {
    #[error("Invalid Hsl: ({0}, {1}, {2})")]
    InvalidHsl(f32, f32, f32)
}


