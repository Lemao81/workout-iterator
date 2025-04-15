use iced::Element;
use iced::widget::{button, center, container};

fn main() -> iced::Result {
    iced::application("Workout Iterator", App::update, App::view)
        .window_size((600.0, 400.0))
        .resizable(false)
        .run()
}

#[derive(Default)]
struct App {}

impl App {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<Message> {
        container(center(button("Next"))).into()
    }
}

#[derive(Debug, Clone)]
enum Message {
    NextWorkout,
    AddWorkoutSelected,
}
