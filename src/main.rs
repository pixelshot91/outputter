use core::str;
use std::{
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
    widgets::{block::Title, Block, Paragraph},
    Terminal,
};
use symbols::border;

struct Burst {
    timestamp: Instant,
    stdout: String,
    stderr: String,
}

fn main() -> Result<()> {
    std::io::stdout().execute(EnterAlternateScreen)?;
       enable_raw_mode()?;
       let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
       terminal.clear()?;
   
    let cmd = Command::new("echo")
        .arg("MyOutput")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let mut cmd_stdout_handle = cmd.stdout.unwrap();
    let mut stderr = cmd.stderr.unwrap();

    let mut buffer: [u8; 100] = [0; 100];


    let mut cmd_bursts: Vec<Burst> = vec![];

    /* loop {
        println!("try reading");
        let res = cmd_stdout.read(&mut buffer[..]).unwrap();
        // dbg!(res);
        sleep(Duration::from_secs(1));
        println!("Got {} bytes: {}", res, str::from_utf8(&buffer[0..res]).unwrap())
    } */

    
    loop {
        let res = cmd_stdout_handle.read(&mut buffer[..]).unwrap();
        // println!("Got {} bytes: {}", res, String::from_utf8(buffer[0..res].to_owned()).unwrap());

        if (res != 0) {
            let stdout = String::from_utf8(buffer[0..res].to_owned()).unwrap();
            let stderr = String::new();
            cmd_bursts.push(Burst {timestamp: Instant::now(), stdout, stderr});
        }

        terminal.draw(|frame| {
            // let area = frame.area();

            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(frame.area());
            // frame.render_widget(Paragraph::new("STDOUT").white().on_blue(), layout[0]);
            let stdout_block = out_block("stdout", &cmd_bursts);
            frame.render_widget(stdout_block, layout[0]);

            // let stderr_block = out_block("stderr");
            // frame.render_widget(stderr_block, layout[1]);

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

fn out_block<'a, 'b, 'c>(title: &'a str, bursts: &'b [Burst]) -> Paragraph<'c> {
    let title = Title::from(format!(" {} ", title).bold());
    let block = Block::bordered()
        .title(title.alignment(Alignment::Center))
        .border_set(border::THICK);
    let stdout_text = Text::from_iter(
        bursts.iter().map(|b| Line::from(vec![b.stdout.clone().into()]))
        // vec![Line::from(vec!["Example".into()])]
    );
    Paragraph::new(stdout_text).centered().block(block)
}
