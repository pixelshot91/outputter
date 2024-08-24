use std::io::{stdout, Result};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    style::Stylize,
    widgets::{block::Title, Block, Paragraph},
    Terminal,
};
use symbols::border;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            // let area = frame.area();

            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(frame.area());
            // frame.render_widget(Paragraph::new("STDOUT").white().on_blue(), layout[0]);
            let stdout_block = out_block("stdout");
            frame.render_widget(stdout_block, layout[0]);

            let stderr_block = out_block("stderr");

            frame.render_widget(stderr_block, layout[1]);

            /*
            let instructions = Title::from(Line::from(vec![
                " Decrement ".into(),
                "<Left>".blue().bold(),
                " Increment ".into(),
                "<Right>".blue().bold(),
                " Quit ".into(),
                "<Q> ".blue().bold(),
            ]));
            .title(
                    instructions
                        .alignment(Alignment::Center)
                        .position(ratatui::widgets::block::Position::Bottom),
                )
             */
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn out_block(title: &str) -> Paragraph {
    let title = Title::from(format!(" {} ", title).bold());
    let block = Block::bordered()
        .title(title.alignment(Alignment::Center))
        .border_set(border::THICK);
    let stdout_text = Text::from(vec![Line::from(vec![
        "Example".into(),
    ])]);
    Paragraph::new(stdout_text)
        .centered()
        .block(block)
}
