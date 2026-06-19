use iced::{
    Element, Length,
    widget::{container, row},
};
use smart_default::SmartDefault;
use strum::{Display, EnumIter};

// Функции в приложении по умолчанию
#[derive(Debug, Clone, Display, EnumIter, PartialEq, Copy)]
pub enum Function {
    #[strum(serialize = "Square (x²)")]
    Square,
    #[strum(serialize = "Bowl (x² + y²)")]
    Bowl,
    #[strum(serialize = "Rosenbrock")]
    Rosenbrock,
    #[strum(serialize = "Himmelblau")]
    Himmelblau,
}

impl Function {
    pub fn formula(&self) -> &'static str {
        match self {
            Function::Square => "f(x) = x²",
            Function::Bowl => "f(x, y) = x² + y²",
            Function::Rosenbrock => "f(x, y) = (1 - x)² + 100(y - x²)²",
            Function::Himmelblau => "f(x, y) = (x² + y - 11)² + (x + y² - 7)³",
        }
    }
}

#[derive(SmartDefault)]
pub struct GradientApp {
    #[default(Function::Square)]
    pub function: Function,
    #[default(0.1)]
    pub learning_rate: f64,
    #[default(3.0)]
    pub initial_point: f64,
    #[default(100.0)]
    pub steps: f64,
}

#[derive(Clone)]
pub enum Message {
    Function(Function),
    LearningRate(f64),
    InitialPoint(f64),
    Steps(f64),
    // NOTE: добавить в будущем события в отдельный enum: Start, Pause, Reset, StepBack, StepForward
}

impl GradientApp {
    pub fn formula_function(&self) -> String {
        self.function.formula().to_string()
    }
}

pub fn update(state: &mut GradientApp, message: Message) {
    match message {
        Message::Function(f) => state.function = f,
        Message::LearningRate(lr) => state.learning_rate = lr,
        Message::InitialPoint(x) => state.initial_point = x,
        Message::Steps(s) => state.steps = s,
    }
}

pub fn view(state: &GradientApp) -> Element<'_, Message> {
    let controls = crate::widgets::widget_data_entry::controls_panel(state);

    let center = container(iced::widget::Space::new())
        .width(Length::Fill)
        .height(Length::Fill);

    row![controls, center].height(Length::Fill).into()
}