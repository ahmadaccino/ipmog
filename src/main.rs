mod app;
mod ip;
mod ui;

use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.on_key('q'),
                        KeyCode::Char('r') => app.on_key('r'),
                        _ => {}
                    }
                }
            }
        }

        app.tick();

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
