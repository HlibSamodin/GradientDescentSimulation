use crate::app::{Function, GradientApp, Message};
use iced::{
    Element, Length,
    widget::{Space, column, container, pick_list, row, slider, text},
};
use strum::IntoEnumIterator;

fn slider_control<'a, F>(
    label_text: &'a str,
    value: f64,
    range: std::ops::RangeInclusive<f64>,
    step: f64,
    on_change: F,
) -> Element<'a, Message>
where
    F: 'a + Fn(f64) -> Message,
{
    column![
        row![text(label_text), text(format!("{:.3}", value))].spacing(10),
        slider(range, value, on_change)
            .step(step)
            .width(Length::Fill),
    ]
    .spacing(8)
    .into()
}

pub fn controls_panel(state: &GradientApp) -> Element<'_, Message> {
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
            Message::LearningRate
        ),
        Space::new().height(Length::Fixed(16.0)),
        slider_control(
            "Initial Point (x₀)",
            state.initial_point,
            -10.0..=10.0,
            0.01,
            Message::InitialPoint
        ),
        Space::new().height(Length::Fixed(16.0)),
        slider_control(
            "Number of Steps",
            state.steps,
            1.0..=1000.0,
            1.0,
            Message::Steps
        ),
    ]
    .spacing(6)
    .padding(16);

    container(panel)
        .width(Length::Fixed(250.0))
        .height(Length::Fill)
        .into()
}