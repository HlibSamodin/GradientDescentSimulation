use iced::widget::{column, container, pick_list, row, slider, text};
use iced::{Background, Color, Element, Length};

// the different functions we can run gradient descent on
// might add more later idk this is just for now
#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    Square,
    Bowl,
    Rosenbrock,
    Himmelblau,
}

impl Function {
    // just the math formula as a string for display
    fn formula(&self) -> &str {
        match self {
            Function::Square     => "f(x) = x²",
            Function::Bowl       => "f(x,y) = x² + y²",
            Function::Rosenbrock => "f(x,y) = (1-x)² + 100(y-x²)²",
            Function::Himmelblau => "f(x,y) = (x²+y-11)² + (x+y²-7)²",
        }
    }

    // this is an actual math takes x,y and returns the value
    // WE WILL USE THEM IN FUTURE FOR THE GRAPH !!!
    fn evaluate(&self, x: f64, y: f64) -> f64 {
        match self {
            Function::Square => x * x,
            Function::Bowl   => x * x + y * y,
            Function::Rosenbrock => {
                // banana function, minimum at (1,1)
                let a = 1.0 - x;
                let b = y - x * x;
                a * a + 100.0 * b * b
            }
            Function::Himmelblau => {
                // has 4 minimums which is kinda cool instead of a single one
                let a = x * x + y - 11.0;
                let b = x + y * y - 7.0;
                a * a + b * b
            }
        }
    }

    // numerical gradient just changes x and y a tiny bit and see what happens
    fn gradient(&self, x: f64, y: f64) -> (f64, f64) {
        let h = 1e-5;
        let dx = (self.evaluate(x + h, y) - self.evaluate(x - h, y)) / (2.0 * h);
        let dy = (self.evaluate(x, y + h) - self.evaluate(x, y - h)) / (2.0 * h);
        (dx, dy)
    }
    // tried to add adaptive learning rate here but it got complicated
    // leaving this for later, not sure if we even need it
    // fn adaptive_lr(&self, grad: f64, prev_grad: f64) -> f64 {
    //     if grad.signum() == prev_grad.signum() {
    //         (self.learning_rate * 1.2).min(10.0)
    //     } else {
    //         self.learning_rate * 0.5
    //     }
    // }
}

// so pick_list can show the name
impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Function::Square     => "x²",
            Function::Bowl       => "x² + y²",
            Function::Rosenbrock => "Rosenbrock",
            Function::Himmelblau => "Himmelblau",
        };
        write!(f, "{}", name)
    }
}

// everything the app needs to remember
pub struct GradientApp {
    function:      Function,
    learning_rate: f64,
    initial_point: f64,
    steps:         f64,
}

impl Default for GradientApp {
    fn default() -> Self {
        Self {
            function:      Function::Square,
            learning_rate: 0.1,
            initial_point: 3.0,
            steps:         100.0,
        }
    }
}

// all the things that can happen in the ui
#[derive(Debug, Clone)]
pub enum Message {
    FunctionChanged(Function),
    LearningRateChanged(f64),
    InitialPointChanged(f64),
    StepsChanged(f64),
    // we will add in future Start, Pause, Reset, StepBack, StepForward here !!!!
}

pub fn update(state: &mut GradientApp, message: Message) {
    match message {
        Message::FunctionChanged(f)      => state.function = f,
        Message::LearningRateChanged(lr) => state.learning_rate = lr,
        Message::InitialPointChanged(x)  => state.initial_point = x,
        Message::StepsChanged(s)         => state.steps = s,
    }
}

// style helpers so we dont repeat colors everywhere

fn panel_bg() -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.12, 0.12, 0.18))),
        ..Default::default()
    }
}

fn center_bg() -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.10, 0.10, 0.15))),
        ..Default::default()
    }
}

// small label above a control
fn label(s: &'static str) -> iced::widget::Text<'static> {
    text(s).size(12).color(Color::from_rgb(0.7, 0.7, 0.8))
}

// even smaller, for hints like the formula
fn hint(s: &'static str) -> iced::widget::Text<'static> {
    text(s).size(12).color(Color::from_rgb(0.5, 0.5, 0.6))
}

// white text for current values
fn value(s: String) -> iced::widget::Text<'static> {
    text(s).size(13).color(Color::WHITE)
}

pub fn view(state: &GradientApp) -> Element<'_, Message> {
    let controls = controls_panel(state);

    // center is empty for now, graph goes here later
    let center = container(iced::widget::Space::new(Length::Fill, Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_| center_bg());

    row![controls, center]
        .height(Length::Fill)
        .into()
}

fn controls_panel(state: &GradientApp) -> Element<'_, Message> {
    let formula = state.function.formula().to_string();

    let panel = column![
        hint("CONTROLS"),

        iced::widget::Space::new(Length::Fill, Length::Fixed(12.0)),

        label("Function"),
        pick_list(
            vec![
                Function::Square,
                Function::Bowl,
                Function::Rosenbrock,
                Function::Himmelblau,
            ],
            Some(state.function.clone()),
            Message::FunctionChanged,
        ).width(Length::Fill),
        value(formula),

        iced::widget::Space::new(Length::Fill, Length::Fixed(16.0)),

        label("Learning Rate (α)"),
        value(format!("{:.3}", state.learning_rate)),
        slider(0.001..=10.0, state.learning_rate, Message::LearningRateChanged)
            .step(0.001)
            .width(Length::Fill),

        iced::widget::Space::new(Length::Fill, Length::Fixed(16.0)),

        label("Initial Point (x₀)"),
        value(format!("{:.4}", state.initial_point)),
        slider(-10.0..=10.0, state.initial_point, Message::InitialPointChanged)
            .step(0.01)
            .width(Length::Fill),

        iced::widget::Space::new(Length::Fill, Length::Fixed(16.0)),

        label("Number of Steps"),
        value(format!("{}", state.steps as u32)),
        slider(1.0..=1000.0, state.steps, Message::StepsChanged)
            .step(1.0)
            .width(Length::Fill),
    ]
    .spacing(6)
    .padding(16);

    container(panel)
        .width(Length::Fixed(250.0))
        .height(Length::Fill)
        .style(|_| panel_bg())
        .into()
}
