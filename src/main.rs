use std::{default, io};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
    player: Player,
}

#[derive(Debug, Default)]
pub struct Player {
    health: u8,
    dead: bool,
    sprite: String,
}

pub const BISHOP: &str = r#"
                  .---.
                 /.'"'.\
                 \\   ||
         /^\    ,_),-','
       .'_|_'.     ()`
      <   |   >    ||
       \_____/     ||
       {/a a\}     ||
      {/-.^.-\}   (_|
     .'{  `  }'-._/|;\
    /  {     }  /; || |
   /`'-{     }-';  || |
  ; `'=|{   }|=' _/|| |
  |   \| |~| |  |/ || |
  |\   \ | | |  ;  || |
  | \   ||=| |=<\  || |
  | /\_/\| | |  \`-||_/
  '-| `;'| | |  |  ||
    |  |+| |+|  |  ||
    |  | | | |  |  ||
    |  """ """  |  ||
    |           |  ||
    |_ _ _ _ _ _|  ||
    |,;,;,;,;,;,|  ||
    `|||||||||||`  ||
     |||||||||||   ||
     `"""""""""`   ""
"#;

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![Constraint::Length(27), Constraint::Percentage(77)])
            .split(frame.area());
        frame.render_widget(self, layout[1]);
        frame.render_widget(&self.player, layout[0]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &Player {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(BISHOP)
            .block(Block::new().borders(Borders::ALL))
            .render(area, buf)
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Myriad".bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
