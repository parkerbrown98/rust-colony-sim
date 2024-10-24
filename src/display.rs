use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{text::Text, CompletedFrame, DefaultTerminal, Frame};
use std::io;

pub struct GameDisplay {
    terminal: DefaultTerminal,
}

impl GameDisplay {
    pub fn new() -> Self {
        GameDisplay {
            terminal: ratatui::init(),
        }
    }

    pub fn start(&mut self) -> io::Result<()> {
        self.terminal.clear()
    }

    pub fn update(&mut self, dt: f32) -> io::Result<CompletedFrame> {
        self.terminal.draw(Self::draw)
    }

    pub fn exit(&mut self) {
        ratatui::restore();
    }

    fn draw(frame: &mut Frame) {
        let text = Text::raw("Hello, Ratatui!");
        frame.render_widget(text, frame.area());
    }
}
