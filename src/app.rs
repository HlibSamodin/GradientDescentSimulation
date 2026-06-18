use iced::widget::{canvas, container, row};
use iced::{Element, Length, Color};
use iced::widget::canvas::{Path, Stroke};
use iced::Point;

#[derive(Default)]
pub struct GradientApp;

#[derive(Debug, Clone)]
pub enum Message {}

pub fn update(_state: &mut GradientApp, _message: Message) {}

pub fn view(_state: &GradientApp) -> Element<Message> {
    let left = container(iced::widget::Space::new(Length::Fill, Length::Fill))
        .width(Length::Fixed(250.0))
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.12, 0.12, 0.18))),
            ..Default::default()
        });

    let center = container(
        canvas(PlotCanvas).width(Length::Fill).height(Length::Fill)
    )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.10, 0.10, 0.15))),
            ..Default::default()
        });

    let right = container(iced::widget::Space::new(Length::Fill, Length::Fill))
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

struct PlotCanvas;

impl<Message> canvas::Program<Message> for PlotCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        _renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(_renderer, bounds.size());

        let w = bounds.width;
        let h = bounds.height;

        let x_min = -3.0_f32;
        let x_max = 3.0_f32;
        let y_min = 0.0_f32;
        let y_max = 9.0_f32;

        let to_screen = |x: f32, y: f32| -> Point {
            Point {
                x: (x - x_min) / (x_max - x_min) * w,
                y: h - (y - y_min) / (y_max - y_min) * h,
            }
        };

        let steps = 300;
        let curve = Path::new(|builder| {
            for i in 0..=steps {
                let t = i as f32 / steps as f32;
                let x = x_min + t * (x_max - x_min);
                let y = x * x; // f(x) = x²
                let p = to_screen(x, y);
                if i == 0 {
                    builder.move_to(p);
                } else {
                    builder.line_to(p);
                }
            }
        });

        frame.stroke(
            &curve,
            Stroke::default()
                .with_color(Color::from_rgb(0.4, 0.8, 1.0))
                .with_width(2.5),
        );

        vec![frame.into_geometry()]
    }
}
