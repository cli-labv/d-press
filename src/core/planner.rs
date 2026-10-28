use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
pub enum OutputMode {
    Flat,
    Preserve,
}

pub struct Plan {
    pub input: PathBuf,
    pub output: PathBuf,
    pub mode: OutputMode,
    pub workers: usize,
}

impl Plan {
    pub fn new(input: impl Into<PathBuf>, output: impl Into<PathBuf>, mode: OutputMode) -> Self {
        Self {
            input: input.into(),
            output: output.into(),
            mode,
            workers: 1,
        }
    }
}
