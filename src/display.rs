use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::*,
    text::Text,
    widgets::{Block, Borders, Paragraph},
    CompletedFrame, DefaultTerminal, Frame,
};
use std::{collections::HashMap, io};

pub struct GameDisplay {
    terminal: DefaultTerminal,
    active_panel: Option<Box<dyn Draw>>,
}

impl GameDisplay {
    pub fn new() -> Self {
        GameDisplay {
            terminal: ratatui::init(),
            active_panel: None,
        }
    }

    pub fn start(&mut self) -> io::Result<()> {
        self.terminal.clear()?;
        let info_panel = InfoPanel {};
        self.set_active_panel(Box::new(info_panel));
        Ok(())
    }

    pub fn update(&mut self) -> io::Result<()> {
        self.terminal.flush()?;
        if let Some(panel) = &mut self.active_panel {
            self.terminal.draw(|frame| {
                panel.draw(frame);
            })?;
        }
        Ok(())
    }

    pub fn exit(&mut self) {
        ratatui::restore();
    }

    pub fn set_active_panel(&mut self, panel: Box<dyn Draw>) {
        self.active_panel = Some(panel);
    }

    // fn draw(frame: &mut Frame) {
    //     let text = Text::raw("Hello, Ratatui!");
    //     frame.render_widget(text, frame.area());
    // }
}

pub trait Draw {
    fn draw(&self, frame: &mut Frame);
}

struct InfoPanel;

impl Draw for InfoPanel {
    fn draw(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.area());

        frame.render_widget(
            Paragraph::new("Left").block(Block::new().borders(Borders::ALL)),
            layout[0],
        );

        frame.render_widget(
            Paragraph::new("Right").block(Block::new().borders(Borders::ALL)),
            layout[1],
        );
    }
}
