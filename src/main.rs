use core::str;
use std::{
    borrow::Cow,
    io::{Read, Result},
    process::{Command, Stdio},
    time::Instant,
};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    style::Stylize,
    widgets::{block::Title, Block, Row, Table},
    Terminal,
};
use symbols::border;

struct Burst {
    timestamp: Instant,
    stdout: String,
    stderr: String,
}

fn main() -> Result<()> {
    let mut args = std::env::args();

    let _this_cmd = args.next();
    let cmd = args.next().expect("No cmd given");
    let cmd_args = args;

    let cmd = Command::new(cmd)
        .args(cmd_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let cmd_start = Instant::now();

    std::io::stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
    terminal.clear()?;

    let mut cmd_stdout_handle = cmd.stdout.unwrap();
    let mut cmd_stderr_handle = cmd.stderr.unwrap();

    let mut stdout_buffer: [u8; 100] = [0; 100];
    let mut stderr_buffer: [u8; 100] = [0; 100];

    let mut cmd_bursts: Vec<Burst> = vec![];

    loop {
        let stdout_bytes_read = cmd_stdout_handle.read(&mut stdout_buffer[..]).unwrap();
        let stderr_bytes_read = cmd_stderr_handle.read(&mut stderr_buffer[..]).unwrap();

        if stdout_bytes_read != 0 || stderr_bytes_read != 0 {
            let stdout = String::from_utf8(stdout_buffer[0..stdout_bytes_read].to_owned()).unwrap();
            let stderr = String::from_utf8(stderr_buffer[0..stderr_bytes_read].to_owned()).unwrap();
            cmd_bursts.push(Burst {
                timestamp: Instant::now(),
                stdout,
                stderr,
            });
        }

        terminal.draw(|frame| {
            let rows = cmd_bursts.iter().map(|b| {
                let duration_since_cmd_start: String =
                    b.timestamp.duration_since(cmd_start).as_secs().to_string();
                let duration_since_cmd_start: Cow<str> = Cow::Owned(duration_since_cmd_start);
                Row::new(vec![
                    duration_since_cmd_start,
                    Cow::Borrowed(&b.stdout),
                    Cow::Borrowed(&b.stderr),
                ])
            });
            // Columns widths are constrained in the same way as Layout...
            let widths = [
                Constraint::Length(10),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ];
            let table = Table::new(rows, widths)
                // ...and they can be separated by a fixed spacing.
                .column_spacing(2)
                // You can set the style of the entire Table.
                // .style(Style::new().blue())
                // It has an optional header, which is simply a Row always visible at the top.
                .header(
                    Row::new(vec!["Timestamp", "Stdout", "Stderr"])
                        .style(Style::new().bold())
                        // To add space between the header and the rest of the rows, specify the margin
                        .bottom_margin(1),
                )
                // It has an optional footer, which is simply a Row always visible at the bottom.
                .footer(Row::new(vec!["Updated on Dec 28"]))
                // As any other widget, a Table can be wrapped in a Block.
                .block(
                    Block::bordered()
                        .title(Title::from("Outputter").alignment(Alignment::Center))
                        .border_set(border::THICK),
                )
                // The selected row and its content can also be styled.
                .highlight_style(Style::new().reversed())
                // ...and potentially show a symbol in front of the selection.
                .highlight_symbol(">>");
            frame.render_widget(table, frame.area());

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

    std::io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
