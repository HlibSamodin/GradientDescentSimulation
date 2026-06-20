use crate::{
    app::{Function, GradientApp, Message},
    themes::ColorScheme,
};
use iced::{
    widget::{column, container, pick_list, row, slider, text, Space},
    Border, Color, Element, Length,
};
use strum::IntoEnumIterator;

fn slider_control<'a, F>(
    label_text: &'a str,
    value: f64,
    range: std::ops::RangeInclusive<f64>,
    step: f64,
    on_change: F,
    color_theme: ColorScheme,
) -> Element<'a, Message>
where
    F: 'a + Fn(f64) -> Message,
{
    column![
        row![text(label_text), text(format!("{:.3}", value))].spacing(10),
        slider(range, value, on_change)
            .step(step)
            .width(Length::Fill)
            .style(move |_theme, status| {
                let radius = if status == slider::Status::Hovered {
                    8.0
                } else {
                    6.0
                };

                slider::Style {
                    rail: slider::Rail {
                        backgrounds: (
                            color_theme.slider_fill.into(),
                            color_theme.slider_track.into(),
                        ),
                        width: 4.0,
                        border: Border {
                            radius: 2.0.into(),
                            width: 0.0,
                            color: Color::TRANSPARENT,
                        },
                    },
                    handle: slider::Handle {
                        shape: slider::HandleShape::Circle { radius },
                        background: color_theme.slider_fill.into(),
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    },
                }
            })
    ]
    .spacing(8)
    .into()
}

pub fn controls_panel(state: &GradientApp) -> Element<'_, Message> {
    let scheme = state.theme.scheme();
    let formula = state.formula_function();
    let options: Vec<Function> = Function::iter().collect();

    let panel = column![
        text("CONTROLS").size(12),
        Space::new().height(Length::Fixed(16.0)),
        text("Function"),
        pick_list(options, Some(state.function), Message::Function,).width(Length::Fill),
        text(formula),
        Space::new().height(Length::Fixed(16.0)),
        slider_control(
            "Learning Rate (α)",
            state.learning_rate,
            0.001..=10.0,
            0.001,
            Message::LearningRate,
            scheme,
        ),
        Space::new().height(Length::Fixed(16.0)),
        slider_control(
            "Initial Point (x₀)",
            state.initial_point,
            -10.0..=10.0,
            0.01,
            Message::InitialPoint,
            scheme,
        ),
        Space::new().height(Length::Fixed(16.0)),
        slider_control(
            "Number of Steps",
            state.steps,
            1.0..=1000.0,
            1.0,
            Message::Steps,
            scheme,
        ),
        Space::new().height(Length::Fixed(16.0)),
        text("THEME").size(12),
        crate::widgets::widget_theme_picker::theme_picker(state.theme),
    ]
    .spacing(6)
    .padding(16);

    container(panel)
        .width(Length::Fixed(250.0))
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(scheme.panel_bg)),
            ..Default::default()
        })
        .into()
}
