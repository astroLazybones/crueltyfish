use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use std::thread;
use std::time::Duration;

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    loop {
        thread::sleep(Duration::from_millis(500));
        enigo.button(Button::Left, Press);
        thread::sleep(Duration::from_secs(1));
        enigo.button(Button::Left, Release);
        thread::sleep(Duration::from_secs(1));
    }
}
