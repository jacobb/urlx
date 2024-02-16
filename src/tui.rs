use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, style::palette::tailwind, widgets::*};
use std::io::{self, Stdout, Write};
use std::process::{Command, Stdio};

struct App {
    items: Vec<String>,
    state: ListState,
}

const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c950;
const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;
const TEXT_COLOR: Color = tailwind::SLATE.c200;

impl App {
    fn new(items: Vec<String>) -> App {
        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0)); // Select the first item
        }
        App { state, items }
    }

    fn prev(&mut self) {
        let selected = self.state.selected().unwrap_or(0);
        if selected > 0 {
            self.state.select(Some(selected - 1));
        }
    }

    fn next(&mut self) {
        let selected = self.state.selected().unwrap_or(0);
        if selected < self.items.len() - 1 {
            self.state.select(Some(selected + 1));
        }
    }

    fn run(mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        loop {
            terminal.draw(|f| self.render_app(f))?;
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    use KeyCode::*;
                    match key.code {
                        Char('q') | Esc => return Ok(()),
                        Char('j') | Down => self.next(),
                        Char('k') | Up => self.prev(),
                        Char('o') | Enter => return self.select(),
                        Char('c') => return self.copy(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn render_app(&mut self, frame: &mut Frame) {
        let list_items: Vec<ListItem> = self
            .items
            .iter()
            .map(|i| ListItem::new(i.as_str()))
            .collect();
        let list = List::new(list_items)
            .block(Block::default().title("URLs").borders(Borders::ALL))
            .fg(TEXT_COLOR)
            .bg(NORMAL_ROW_COLOR)
            .highlight_style(
                Style::new()
                    .add_modifier(Modifier::REVERSED)
                    .fg(SELECTED_STYLE_FG),
            )
            .highlight_symbol("🐧 ")
            .repeat_highlight_symbol(true);

        frame.render_stateful_widget(list, frame.size(), &mut self.state);
    }

    fn select(&self) -> Result<()> {
        let selected = self.state.selected().unwrap_or(0);
        let url = &self.items[selected];

        let mut child = Command::new("open").arg(url).spawn()?;
        child.wait()?; // Wait for the child process to exit.
        Ok(())
    }

    fn copy(&self) -> Result<()> {
        let selected = self.state.selected().unwrap_or(0);
        let url = &self.items[selected];

        let mut child = Command::new("pbcopy").stdin(Stdio::piped()).spawn()?;

        if let Some(ref mut stdin) = child.stdin.take() {
            writeln!(stdin, "{}", url).expect("Failed to write to child stdin");
        } else {
            return Err(anyhow::Error::msg(
                "Child process stdin has not been captured",
            ));
        }

        child.wait()?; // Wait for the child process to exit.
        Ok(())
    }
}

pub fn choose(urls: Vec<String>) -> Result<()> {
    let app = App::new(urls);
    let mut terminal = setup_terminal().context("setup failed")?;
    app.run(&mut terminal).context("app loop failed")?;
    restore_terminal(&mut terminal).context("restore terminal failed")?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
}
