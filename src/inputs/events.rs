use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::thread;
use std::time::Duration;
use std::io;

// use termion::event::Key;
use termion::input::TermRead;
use super::key::Key;
use super::InputEvent;

pub struct Events {
    rx: Receiver<InputEvent>,
    // Need to be kept around to prevent disposing the sender side.
    _tx: Sender<InputEvent>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Events {
        let (tx, rx) = channel();

        let event_tx = tx.clone(); // the thread::spawn own event_tx 
        thread::spawn(move || {
            loop {
                // poll for tick rate duration, if no event, sent tick event.
                for evt in io::stdin().keys() {
                    if let Ok(key) = evt {
                        event_tx.send(InputEvent::Input(Key(key))).unwrap();
                    }
                }
                event_tx.send(InputEvent::Tick).unwrap();
            }
        });

        let event_tx = tx.clone();
        thread::spawn(move || loop {
            if let Err(err) = event_tx.send(InputEvent::Tick) {
                eprintln!("{}", err);
                break;
            }
            thread::sleep(tick_rate);
        });

        Events { rx, _tx: tx }
    }

    /// Attempts to read an event.
    /// This function block the current thread.
    pub fn next(&self) -> Result<InputEvent, RecvError> {
        self.rx.recv()
    }
}
