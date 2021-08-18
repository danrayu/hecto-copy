use std::error::Error;
use std::io::{self, Read, stdin, stdout};       
use termion::raw::IntoRawMode;     
use termion::event::Key;
use termion::input::TermRead;

use crate::main;

pub struct Editor {
    quit_ordered: bool
}

impl Editor {

    pub fn run(&self) {
        let std_out = stdout().into_raw_mode().unwrap();
        loop {
            self.process_keypress();
        }
    }

    fn process_keypress(&self) -> Result<(), std::io::Error> {
        let pressed_key = self.read_key()?;
        match pressed_key {
            Key::Ctrl('q') => panic!("Exiting program."),
            _ => (),
        }
        Ok(())
    }

    fn read_key(&self) -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn default() -> Self{
        Self {quit_ordered: false}
    }
}