mod helper;
mod persistence;
mod ui;

use crate::persistence::read_workouts_state;
use crate::ui::{ViewModel, create_view};
use iced::{Element, Task};
use serde::{Deserialize, Serialize};

fn main() -> iced::Result {
    let workouts_state = read_workouts_state();
    let app = App {
        workouts: workouts_state.workouts,
        index: workouts_state.index,
    };

    iced::application("Workout Iterator", App::update, App::view)
        .window_size((500.0, 300.0))
        .resizable(false)
        .run_with(|| (app, Task::none()))
}

struct App {
    index: i8,
    workouts: Vec<String>,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::NextWorkout => self.on_next_workout(),
            _ => return,
        }
    }

    fn on_next_workout(&mut self) {
        let count = self.workouts.iter().count() as i8;
        if count > 0 {
            self.index = (self.index + 1) % count;
        }
    }

    fn view(&self) -> Element<Message> {
        let workout = self
            .workouts
            .iter()
            .nth(self.index as usize)
            .unwrap_or(&"<empty>".to_owned())
            .clone();
        let total = self.workouts.iter().count();
        let has_next = total > 1;
        let selected_number = if total == 0 { 0 } else { self.index + 1 };

        create_view(ViewModel {
            workout,
            has_next,
            selected_number,
            total,
        })
        .into()
    }
}

#[derive(Debug, Clone)]
enum Message {
    NextWorkout,
    AddWorkoutSelected,
}

#[derive(Serialize, Deserialize, Default)]
struct WorkoutsState {
    index: i8,
    workouts: Vec<String>,
}
