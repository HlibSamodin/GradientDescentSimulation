use iced::widget::{canvas, container, row};
use iced::{Element, Length, Color};

#[derive(Default)]
pub struct GradientApp;

#[derive(Debug, Clone)]
pub enum Message {}

pub fn update(_state: &mut GradientApp, _message: Message) {}

pub fn view(_state: &GradientApp) -> Element<Message> {
    // left panel
    let left = container(canvas(PlotCanvas).width(Length::Fill).height(Length::Fill))
        .width(Length::Fixed(250.0))
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.12, 0.12, 0.18))),
            ..Default::default()
        });

    // center
    let center = container(canvas(PlotCanvas).width(Length::Fill).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.10, 0.10, 0.15))),
            ..Default::default()
        });

    // right panel
    let right = container(canvas(PlotCanvas).width(Length::Fill).height(Length::Fill))
        .width(Length::Fixed(280.0))
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.12, 0.12, 0.18))),
            ..Default::default()
        });

    row![left, center, right]
        .height(Length::Fill)
        .into()
}

// blank
struct PlotCanvas;

impl<Message> canvas::Program<Message> for PlotCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        _renderer: &iced::Renderer,
        _theme: &iced::Theme,
        _bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        vec![]
    }
}
