use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{self};
use crossterm::{cursor, execute};
use std::io::{self, Stdout};
use std::time::Duration;

struct Terminal {
    stdout: Stdout,
}

impl Terminal {
    pub fn default() -> Self {
        Self {
            stdout: io::stdout(),
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

    fn draw_screen(&mut self, text: &String) -> io::Result<()> {
        execute!(
            self.stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
            Print(text)
        )?;
        Ok(())
    }
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    buffer: Buffer,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default(),
            buffer: Buffer::default(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.terminal.setup()?;
        while !self.should_quit {
            if poll(Duration::from_millis(500))? {
                let event = read()?;
                self.handle_input(&event);
                self.terminal.draw_screen(&self.buffer.render())?;
            }
        }
        self.terminal.clean_up()?;
        Ok(())
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
            _ => print!("Unsupported event \r"),
        }
    }
}

struct Buffer {
    main_text: String,
    current: String,
}

impl Buffer {
    pub fn default() -> Self {
        Self {
            main_text: String::new(),
            current: String::new(),
        }
    }

    pub fn add_char(&mut self, c: char) {
        if c == ' ' {
            self.current.push(c);
            self.main_text.push_str(&self.current);
            self.current.clear();
        } else{
            self.current.push(c);
        }
    }

    pub fn del_char(&mut self) {
        if self.current.is_empty(){
            self.main_text.pop();
        } else {
            self.current.pop();
        }
    }

    pub fn del_word(&mut self) {
        if self.current.is_empty(){
            let mut space_counter = 0;
            loop {
                let Some(c) = self.main_text.pop() else { break };
                if c == ' ' {
                    space_counter += 1;
                };
                if space_counter == 2{
                    self.main_text.push(' ');
                    break;
                }
            }
        } else {
            self.current.clear();
        }
    }

    pub fn render(&mut self) -> String {
        let render = format!("{}{}", self.main_text, self.current);
        render
    }
}
