use iced::widget::text;
use iced::Element;

#[derive(Default)]
pub struct GradientApp;

#[derive(Debug, Clone)]
pub enum Message {}

pub fn update(_state: &mut GradientApp, _message: Message) {}

pub fn view(_state: &GradientApp) -> Element<Message> {
    text("Window is working").into()
}
