use iced::{Color, Element, Length};
use iced::widget::{button, container, row};

// темы
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Purple,
    Blue,
    Green,
    Yellow,
}

impl Theme {
    fn color(&self) -> Color {
        match self {
            Theme::Purple => Color::from_rgb(0.5, 0.2, 0.8),
            Theme::Blue   => Color::from_rgb(0.2, 0.4, 0.9),
            Theme::Green  => Color::from_rgb(0.2, 0.7, 0.4),
            Theme::Yellow => Color::from_rgb(0.9, 0.7, 0.1),
        }
    }

    fn all() -> &'static [Theme] {
        &[Theme::Purple, Theme::Blue, Theme::Green, Theme::Yellow]
    }
}

pub fn theme_picker(selected: Theme) -> Element<'static, crate::app::Message> {
    let circles = Theme::all().iter().map(|t| {
        let color = t.color();
        let is_selected = *t == selected;

        let dot = container(iced::widget::Space::new())
            .width(Length::Fixed(28.0))
            .height(Length::Fixed(28.0))
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(color)),
                border: iced::Border {
                    radius: 14.0.into(),
                    width: if is_selected { 2.5 } else { 0.0 },
                    color: Color::WHITE,
                },
                ..Default::default()
            });

        button(dot)
            .on_press(crate::app::Message::ThemeChanged(*t))
            .style(|_, _| button::Style {
                background: None,
                ..Default::default()
            })
            .into()
    }).collect::<Vec<_>>();

    row(circles).spacing(8).into()
}
