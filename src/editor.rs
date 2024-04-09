use std::char;
use std::io::{self, Stdout};
use std::time::{Duration, Instant};

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{self};
use crossterm::{cursor, execute};

pub struct Editor {
    should_quit: bool,
    stdout: Stdout,
    buffer: Buffer,
    start_time: Instant,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            stdout: io::stdout(),
            buffer: Buffer::default(),
            start_time: Instant::now(),
        }
    }

    fn setup(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(
            self.stdout,
            terminal::EnterAlternateScreen,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        Ok(())
    }

    fn clean_up(&mut self) -> io::Result<()> {
        execute!(self.stdout, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn calculate_wpm(&self) -> String {
        format!(
            "WPM: {}",
            ((self.buffer.lenght() as f64 / 5.0)
                / (self.start_time.elapsed().as_secs() as f64 / 60.0)) as i32
        )
    }

    fn handle_input(&mut self, event: &Event) {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => self.should_quit = true,
            Event::Key(KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => self.buffer.del_word(),
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) => self.buffer.del_char(),
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => self.buffer.add_char(*c),
            _ => (),
        }
    }

    fn draw_screen(&mut self, text: &String, wpm: String) -> io::Result<()> {
        execute!(
            self.stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
            Print(wpm),
            cursor::MoveTo(0, 1),
            Print(text),
        )?;
        Ok(())
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.setup()?;
        while !self.should_quit {
            if poll(Duration::from_millis(500))? {
                let event = read()?;
                self.handle_input(&event);
                self.draw_screen(&self.buffer.render(), self.calculate_wpm())?;
            }
        }
        self.clean_up()?;
        Ok(())
    }
}

struct Buffer {
    main_text: String,
}

impl Buffer {
    pub fn default() -> Self {
        Self {
            main_text: String::new(),
        }
    }

    pub fn add_char(&mut self, c: char) {
        self.main_text.push(c);
    }

    pub fn del_char(&mut self) {
        self.main_text.pop();
    }

    pub fn del_word(&mut self) {
        if self.main_text.ends_with(' ') {
            self.main_text.pop();
        }
        loop {
            let Some(c) = self.main_text.pop() else { break };
            if c == ' ' {
                self.main_text.push(' ');
                break;
            };
        }
    }

    pub fn render(&self) -> String {
        self.main_text.clone()
    }

    pub fn lenght(&self) -> usize {
        self.main_text.len()
    }
}
