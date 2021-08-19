use std::env;
use crate::document::{Document, Row};
use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_default()
        } else {
            Document::default()
        };
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            document
        }
    }

    fn draw_row(&self, row: &Row) {
        println!("{}\r", row.render(0, self.terminal.size().width as usize));

    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn move_cursor(&mut self, key: Key) {
        let Position {mut x, mut y} = self.cursor_position;
        match key {
            Key::Left => x = x.saturating_sub(1),
            Key::Right => x = x.saturating_add(1),
            Key::Up => y = y.saturating_sub(1),
            Key::Down => y = y.saturating_add(1),
            _ => ()
        }
        self.cursor_position = Position { x, y }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn draw_welcome_message(&self) {
        let mut message = format!("Hecto editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;    
        let padding = " ".repeat(width.overflowing_sub(message.len()).0 / 2);
        message.truncate(width);
        println!("~{}{}\r", padding, message)
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.get_row(terminal_row as usize) {
                self.draw_row(row)
            } else if self.document.is_empty() {
                if terminal_row == height/3 {
                    self.draw_welcome_message();
                }
            }
            else {
                println!("~\r");
            }
        }
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!(e);
}