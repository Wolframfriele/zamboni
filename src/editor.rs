use std::io::{self, Stdout};
use std::time::Duration;
use crossterm::{execute, queue, cursor};
use crossterm::terminal::{self};
use crossterm::event::{read, Event, KeyEvent, KeyCode, KeyModifiers, poll};

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn default() -> Self {
        Self { 
            should_quit: false, 
            terminal: Terminal::default().expect("Failed to get terminal size")
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.terminal.setup()?;
        while !self.should_quit {
            if poll(Duration::from_millis(500)) ?{
                let event = read()?;
                self.handle_input(&event);
            }
        }
        self.terminal.clean_up()?;
        Ok(())
    }
    
    fn handle_input(&mut self, event: &Event) {
        match event {
            Event::Key(KeyEvent {code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL, ..}) => self.should_quit = true, 
            Event::Key(event)=> println!("{event:?} \r"),
            _ => print!("Unsupported event \r"),
        }
    }
    // fn draw_rows(&self) {
    //     for _ in 0..self.terminal.size().height {
    //         println!("~\r");
    //     }
    // }

   

   
}

struct Terminal {
    window_size: terminal::WindowSize,
    stdout: Stdout,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let window_size_result = terminal::window_size()?;
        Ok(Self {
            window_size: window_size_result,
            stdout: io::stdout(),
        })
    }

    pub fn size(&self) -> &terminal::WindowSize {
        &self.window_size 
    }

    fn setup(&mut self) -> io::Result<()> {
        execute!(self.stdout, terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        queue!(
            self.stdout, 
            terminal::Clear(terminal::ClearType::All), 
            cursor::MoveTo(0, 0)
        )?;
        execute!(self.stdout, cursor::MoveTo(0, 0))?;
        Ok(())
    }

    fn clean_up(&mut self) -> io::Result<()> {
        queue!(self.stdout, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}
