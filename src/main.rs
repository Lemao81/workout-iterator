use iced::widget::{Column, button, center, container, text};
use iced::{Element, Padding};

fn main() -> iced::Result {
    iced::application("Workout Iterator", App::update, App::view)
        .window_size((500.0, 300.0))
        .resizable(false)
        .run()
}

struct App {
    workouts: Vec<&'static str>,
    index: usize,
}

impl Default for App {
    fn default() -> Self {
        let workouts = vec![
            "Lorem ipsum dolor sit amet, consetetur sadipscing",
            "Workout 2",
        ];

        App { workouts, index: 0 }
    }
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::NextWorkout => self.index = (self.index + 1) % self.workouts.iter().count(),
            _ => return,
        }
    }

    fn view(&self) -> Element<Message> {
        let center_width = 250.0;
        let center_height = 100.0;

        let workout_txt =
            center(text(*self.workouts.iter().nth(self.index).unwrap_or(&"")).size(28))
                .width(center_width)
                .height(center_height);
        let next_btn = center(
            button("Next")
                .on_press(Message::NextWorkout)
                .padding(Padding::from([16.0, 28.0])),
        )
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
