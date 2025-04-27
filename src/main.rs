mod helper;
mod persistence;
mod ui;

use crate::persistence::read_workouts_state;
use crate::ui::settings_page::{SettingsViewModel, create_settings_page};
use crate::ui::{MainViewModel, Page, WINDOW_HEIGHT, WINDOW_WIDTH, create_main_page};
use iced::{Element, Task};
use serde::{Deserialize, Serialize};

fn main() -> iced::Result {
    let workouts_state = read_workouts_state();
    let app = App {
        workouts: workouts_state.workouts,
        index: workouts_state.index,
        current_page: Page::Main,
    };

    iced::application("Workout Iterator", App::update, App::view)
        .window_size((WINDOW_WIDTH, WINDOW_HEIGHT))
        .resizable(false)
        .run_with(|| (app, Task::none()))
}

struct App {
    index: i8,
    workouts: Vec<String>,
    current_page: Page,
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
        self.current_page = Page::Settings;
    }

    fn on_close_settings(&mut self) {
        self.current_page = Page::Main;
    }

    fn view(&self) -> Element<Message> {
        match self.current_page {
            Page::Main => create_main_page(self.create_main_view_model()).into(),
            Page::Settings => create_settings_page(self.create_settings_view_model()).into(),
        }
    }

    fn create_main_view_model(&self) -> MainViewModel {
        let workout = self
            .workouts
            .iter()
            .nth(self.index as usize)
            .unwrap_or(&"<empty>".to_owned())
            .clone();
        let total = self.workouts.iter().count();
        let has_next = total > 1;
        let selected_number = if total == 0 { 0 } else { self.index + 1 };

        MainViewModel {
            workout,
            has_next,
            selected_number,
            total,
        }
    }

    fn create_settings_view_model(&self) -> SettingsViewModel {
        SettingsViewModel {
            workouts: self.workouts.clone(),
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
