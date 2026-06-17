mod app;

fn main() -> iced::Result {
    iced::run("Gradient Descent Simulation", app::update, app::view)
}
