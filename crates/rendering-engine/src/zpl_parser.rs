use std::path::Path;

use crate::{CurrentState, ParseError};

pub struct ZPLParser<'a> {
    state: CurrentState<'a>,
}

impl<'a> ZPLParser<'a> {
    pub fn new(density: usize, width: usize, height: usize) -> Self {
        let state = CurrentState::new(density, width, height);
        Self { state }
    }

    pub fn resize(&mut self, density: usize, width: usize, height: usize) {
        self.state = CurrentState::new(density, width, height);
    }

    pub fn process_format(&mut self, format: &'a str) -> Result<(), ParseError> {
        self.state.process_format(format)
    }

    pub fn render(&mut self) {
        self.state.render()
    }

    pub fn to_png(&mut self, path: impl AsRef<Path>) {
        self.state.to_png(path)
    }

    pub fn to_jpg(&mut self, path: impl AsRef<Path>) {
        self.state.to_jpg(path)
    }

    pub fn to_zpl_code(&self) -> String {
        self.state.to_zpl_code()
    }
}
