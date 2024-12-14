use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Direction, Layout},
    text::Text,
    widgets::Paragraph,
    DefaultTerminal, Frame,
};

struct App {
    count: i64,
}

const PROBLEM_INPUT: &str = include_str!("../../data/day14.txt");

impl App {
    fn new() -> Self {
        Self { count: 0 }
    }

    fn draw(&self, frame: &mut Frame) {
        use aoc_2024::day14::SecurityTeam;

        let dimensions = (103, 101);

        let mut bots = SecurityTeam::new(PROBLEM_INPUT, dimensions);
        bots.timeshift(self.count);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(1), Constraint::Percentage(99)])
            .split(frame.area());

        let minutes = Paragraph::new(format!("Minutes: {}", self.count));
        frame.render_widget(minutes, layout[0]);

        let map = Text::raw(bots.to_string());
        frame.render_widget(map, layout[1]);
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Left => self.count -= 1,
                    KeyCode::Right => self.count += 1,
                    KeyCode::PageUp => self.count -= 100,
                    KeyCode::PageDown => self.count += 100,
                    _ => {
                        return Ok(());
                    }
                }
            }
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?; // augment errors / panics with easy to read messages
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}
