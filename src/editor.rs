use std::cmp::min;
use std::{char, fs};
use std::io::{self, Stdout};
use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{self};
use crossterm::{cursor, execute};

use inflector::Inflector;

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
    spellcheck: Spellcheck,
    capitalize_next: bool,
}

impl Buffer {
    pub fn default() -> Self {
        Self {
            main_text: String::new(),
            current: String::new(),
            spellcheck: Spellcheck::default(),
            capitalize_next: true,
        }
    }

    pub fn add_char(&mut self, c: char) {
        if c == ' ' || c == ',' || c == ':' || c == ';' {
            let mut word = self.spellcheck.find_corrections(&self.current.to_lowercase());
            if self.capitalize_next && self.current.len() > 1 {
                word = word.to_title_case();
                self.capitalize_next = false;
            } 
            self.main_text.push_str(&word);
            self.main_text.push(c);
            self.current.clear();
        } else if c == '.' || c == '?' || c == '!' {
            self.main_text.push_str(&self.spellcheck.find_corrections(&self.current.to_lowercase()));
            self.main_text.push(c);
            self.current.clear();
            self.capitalize_next = true;
        } else {
            self.current.push(c);
        }
    }

    pub fn del_char(&mut self) {
        if self.current.is_empty() {
            self.main_text.pop();
        } else {
            self.current.pop();
        }
    }

    pub fn del_word(&mut self) {
        if self.current.is_empty() {
            let mut space_counter = 0;
            loop {
                let Some(c) = self.main_text.pop() else { break };
                if c == ' ' {
                    space_counter += 1;
                };
                if space_counter == 2 {
                    self.main_text.push(' ');
                    break;
                }
            }
        } else {
            self.current.clear();
        }
    }

    pub fn render(&mut self) -> String {
        format!("{}{}", self.main_text, self.current)
    }
}

struct Spellcheck {
    corpus: Vec<String>,
}

impl Spellcheck {
    pub fn default() -> Self {
        Self {
            corpus: fs::read_to_string("b.txt")
                .expect("Could not read the wordlist.")
                .split_whitespace()
                .map(str::to_string)
                .collect(),
        }
    }

    fn edit_distance(word1: &String, word2: &String) -> usize {
        if word1 == word2 {
            return 0
        }
        let (word1, word2) = (word1.as_bytes(), word2.as_bytes());
        
        let mut current: Vec<usize> = (0..=word1.len()).collect();
        let mut previous = current.clone();

        for i in 1..=word2.len() {
            previous.copy_from_slice(&current);
            current[0] = i;
            for j in 1..=word1.len() {
                let mut min_cost = previous[j - 1];

                if word1[j - 1] != word2[i - 1] {
                    let insert = previous[j];
                    let replace = previous[j - 1];
                    let delete = current[j - 1];

                    min_cost = min(insert, min(replace, delete)) + 1;
                }
                current[j] = min_cost;
            }
        }
        current[word1.len()]
    }

    fn similarity_score(word1: &String, word2: &String) -> f32 {
        1. - (Self::edit_distance(word1, word2)) as f32 / min(word1.len(), word2.len()) as f32
    }

    pub fn find_corrections(&self, word: &String) -> String {
        let word_len = word.chars().count() as i32;
        let mut closest_word = word.clone();
        let mut closest_distance: f32 = 0.0;

        for dict_word in &self.corpus {
            if (dict_word.chars().count() as i32 - word_len).abs() <= 3{
                let score = Self::similarity_score(dict_word, word);
                if score > closest_distance {
                    closest_distance = score;
                    closest_word = dict_word.clone();
                }
            }
        }
        if closest_word == "i" {
            return String::from("I")
        }
        closest_word.to_string()
    }
}

