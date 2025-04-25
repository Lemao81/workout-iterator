mod helper;
mod persistence;
mod ui;

use crate::helper::modal;
use crate::persistence::read_workouts_state;
use crate::ui::settings_page::create_settings_page;
use crate::ui::{ViewModel, create_view};
use iced::{Element, Task};
use serde::{Deserialize, Serialize};

fn main() -> iced::Result {
    let workouts_state = read_workouts_state();
    let app = App {
        workouts: workouts_state.workouts,
        index: workouts_state.index,
        show_settings_page: false,
    };

    iced::application("Workout Iterator", App::update, App::view)
        .window_size((500.0, 300.0))
        .resizable(false)
        .run_with(|| (app, Task::none()))
}

struct App {
    index: i8,
    workouts: Vec<String>,
    show_settings_page: bool,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::NextWorkout => self.on_next_workout(),
            Message::OpenSettings => self.on_open_settings(),
            Message::CloseSettings => self.on_close_settings(),
        }
    }

    fn on_next_workout(&mut self) {
        let count = self.workouts.iter().count() as i8;
        if count > 0 {
            self.index = (self.index + 1) % count;
        }
    }

    fn on_open_settings(&mut self) {
        self.show_settings_page = true;
    }

    fn on_close_settings(&mut self) {
        self.show_settings_page = false;
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

        let main_page = create_view(ViewModel {
            workout,
            has_next,
            selected_number,
            total,
        })
        .into();

        if self.show_settings_page {
            modal(main_page, create_settings_page(), Message::CloseSettings)
        } else {
            main_page
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    NextWorkout,
    OpenSettings,
    CloseSettings,
}

#[derive(Serialize, Deserialize, Default)]
struct WorkoutsState {
    index: i8,
    workouts: Vec<String>,
}
