use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::io;

pub struct MoonSystem {
    phase: f32, // 0.0 - 1.0 (New - Full - New)
    x: u16,
    y: u16,
}

impl MoonSystem {
    pub fn new(terminal_width: u16, terminal_height: u16) -> Self {
        Self {
            phase: 0.5, // TODO: fetch from API?
            x: (terminal_width / 4) + 10,
            y: (terminal_height / 4) + 2,
        }
    }

    pub fn update(&mut self, _terminal_width: u16, _terminal_height: u16) {
        // Moon moves VERY slowly or stays fixed.
    }

    pub fn render(&self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        let moon_art = match self.phase {
            _ => vec!["  _.._  ", " .'.''. ", "( :  : )", " '....' ", "  `''`  "],
        };

        for (i, line) in moon_art.iter().enumerate() {
            renderer.render_line_colored(self.x, self.y + i as u16, line, Color::White)?;
        }
        Ok(())
    }
}
