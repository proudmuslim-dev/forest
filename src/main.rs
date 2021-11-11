use forest::ui::forest::*;
use iced::{Application, Settings};

pub fn main() -> iced::Result {
    Forest::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}
