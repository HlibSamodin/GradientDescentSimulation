mod app;
mod widgets;
use app::{update, view};

fn main() -> iced::Result {
    iced::run(update, view)
}