mod app;
mod widgets;
mod themes;
use app::{update, view};

fn main() -> iced::Result {
    iced::run(update, view)
}