use std::fmt::{self, Display, Formatter};
use termion::event;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Key(pub event::Key);

impl Display for Key {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.0 {
            event::Key::Alt(' ') => write!(f, "<Alt+Space>"),
            event::Key::Ctrl(' ') => write!(f, "<Ctrl+Space>"),
            event::Key::Char(' ') => write!(f, "<Space>"),
            event::Key::Alt(c) => write!(f, "<Alt+{}>", c),
            event::Key::Ctrl(c) => write!(f, "<Ctrl+{}>", c),
            event::Key::Char(c) => write!(f, "<{}>", c),
            _ => write!(f, "<{:?}>", self),
        }
    }
}
