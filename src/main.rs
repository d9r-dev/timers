mod app;
mod ui;

use crate::app::{App, CurrentScreen};
use crate::ui::ui;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{event, execute};
use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};
use std::error::Error;
use std::io;
use std::io::Stderr;
use std::task::Wake;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut terminal, mut app) = initialize_app()?;
    //timers for demoing
    create_test_data(&mut app);

    run_app(&mut terminal, &mut app);

    // restore terminal
    restore_terminal(&mut terminal);

    Ok(())
}

fn initialize_app() -> Result<(Terminal<CrosstermBackend<Stderr>>, App), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    Ok((terminal, app))
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stderr>>) {
    disable_raw_mode().expect("Unable to disable raw mode");
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor().expect("Unable to show cursor");
}

fn create_test_data(app: &mut App) {
    let mut timer1 = app::Timer::new("Test1", "Lorem ipsum", 1);
    timer1.stop();
    let mut timer2 = app::Timer::new("Test2", "Lorem ipsum", 2);
    timer2.stop();
    let mut timer3 = app::Timer::new("Test3", "Lorem ipsum", 3);
    timer3.stop();
    let timer4 = app::Timer::new("Test4", "Lorem ipsum", 4);
    app.timers.push(timer1);
    app.timers.push(timer2);
    app.timers.push(timer3);
    app.timers.push(timer4);
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(16);
    loop {
        if let Some(last_timer) = app.timers.last_mut() {
            last_timer.set_duration()
        }
        terminal.draw(|f| ui(f, app))?;
        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                match app.current_screen {
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('q') => {
                            app.current_screen = CurrentScreen::Exit;
                        }
                        KeyCode::Char('j') => {
                            app.next_row();
                        }
                        KeyCode::Char('k') => {
                            app.previous_row();
                        }
                        _ => {}
                    },
                    CurrentScreen::Exit => match key.code {
                        KeyCode::Char('q') | KeyCode::Char('n') => {
                            app.current_screen = CurrentScreen::Main;
                        }
                        KeyCode::Char('y') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                }
            }
        } else {
            continue;
        }
    }
}
