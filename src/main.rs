// tuipdf
// ------
// A beautifully crafted, terminal-native PDF tool built in Rust.
// It aims to make compressing PDF files as fast, efficient and flexible
// as possible directly from your terminal.
//
// Authors: KnightShadows Team and individual contributors (see CONTRIBUTORS file)
//          Aditya Anand <aditya19study@gmail.com> (c) 2025
// Website: https://github.com/KnightShadows/tuipdf
// License: MPL-2.0 (see LICENSE file)

mod app;
mod compression;
mod input;
mod ui;

use std::io;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::Parser;
use crossterm::{
    event::{self, DisableBracketedPaste, EnableBracketedPaste, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use app::{Focus, Status};

#[derive(Parser, Debug)]
#[command(name = "tuipdf", version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "FILE")]
    file: Option<String>,

    #[arg(short, long, value_name = "INSTRUCTION")]
    prompt: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stderr(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));

    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableBracketedPaste)
        .context("Failed to enter alternate screen")?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;
    terminal.clear()?;

    let mut app = app::App::new();

    if let Some(ref path) = cli.file {
        app.input_field.paste(path);
        app.derive_output_path();
    }

    if let Some(ref prompt) = cli.prompt {
        app.prompt = Some(prompt.clone());
    }

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        app.poll_compression();
        app.tick();

        if event::poll(Duration::from_millis(50)).context("Event poll failed")? {
            let ev = event::read().context("Event read failed")?;

            if let Event::Paste(ref text) = ev {
                if app.focus == Focus::InputPath && !matches!(app.status, Status::Compressing) {
                    let inner_width = terminal
                        .size()
                        .map(|s| s.width.saturating_sub(4) as usize)
                        .unwrap_or(40);
                    app.input_field.paste(text);
                    app.input_field.clamp_scroll(inner_width);
                    app.derive_output_path();
                }
                continue;
            }

            if let Event::Key(key) = ev {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match &app.status {
                    Status::Error(_) => {
                        app.status = Status::Idle;
                        app.progress = 0.0;
                        continue;
                    }

                    Status::Compressing => {
                        if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL)) {
                            break;
                        }
                        continue;
                    }

                    _ => {}
                }

                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        break;
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Char('q') if app.focus != Focus::InputPath => {
                        break;
                    }
                    KeyCode::Tab => {
                        app.cycle_focus();
                    }
                    KeyCode::Enter => {
                        if app.focus == Focus::CompressionLevel {
                            app.compression_level = app.compression_level.next();
                        } else {
                            app.derive_output_path();
                            if let Some(err_msg) = app.try_compress() {
                                app.status = Status::Error(err_msg);
                            }
                        }
                    }
                    _ => {
                        if app.focus == Focus::InputPath {
                            let inner_width = terminal
                                .size()
                                .map(|s| s.width.saturating_sub(4) as usize)
                                .unwrap_or(40);
                            app.input_field.handle_event(key);
                            app.input_field.clamp_scroll(inner_width);
                            app.derive_output_path();
                        }
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    if let Some(handle) = app.compression_handle.take() {
        let _ = handle.join();
    }

    disable_raw_mode().context("Failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableBracketedPaste
    )
    .context("Failed to leave alternate screen")?;
    terminal.show_cursor().context("Failed to show cursor")?;

    Ok(())
}