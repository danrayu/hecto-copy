mod editor;

use std::io::{self, Read, stdin, stdout};       
use termion::raw::IntoRawMode;     
use termion::event::Key;
use termion::input::TermRead;
use editor::Editor;

fn main() {
    let editor = Editor::default();
    editor.run();
}