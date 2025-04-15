use iced::widget::{button, center, container, text, Column};
use iced::{Element, Padding};

fn main() -> iced::Result {
    iced::application("Workout Iterator", App::update, App::view)
        .window_size((500.0, 300.0))
        .resizable(false)
        .run()
}

#[derive(Default)]
struct App {}

impl App {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<Message> {
        let center_width = 250.0;
        let center_height = 100.0;

        let workout_txt =
            center(text("Lorem ipsum dolor sit amet, consetetur sadipscing").size(28))
                .width(center_width)
                .height(center_height);
        let next_btn = center(button("Next").padding(Padding::from([16.0, 28.0])))
            .width(center_width)
            .height(center_height);
        let column = Column::with_children(vec![workout_txt.into(), next_btn.into()]);

        container(center(column)).into()
    }
}

#[derive(Debug, Clone)]
enum Message {
    NextWorkout,
    AddWorkoutSelected,
}
