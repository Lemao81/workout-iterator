mod helper;
mod persistence;
mod ui;

use crate::persistence::{log_error, read_workouts_state, write_workouts_state};
use crate::ui::settings_page::{SettingsViewModel, create_settings_page};
use crate::ui::{MainViewModel, Page, WINDOW_HEIGHT, WINDOW_WIDTH, create_main_page};
use iced::{Element, Task};
use serde::{Deserialize, Serialize};

fn main() -> iced::Result {
    let workouts_state = read_workouts_state();
    let app_state = AppState {
        workout_index: workouts_state.index,
        workouts: workouts_state.workouts,
        current_page: Page::Main,
        workout_selection: None,
        workout_input: None,
        can_add: false,
    };

    iced::application("Workout Iterator", AppState::update, AppState::view)
        .window_size((WINDOW_WIDTH, WINDOW_HEIGHT))
        .resizable(false)
        .run_with(|| (app_state, Task::none()))
}

struct AppState {
    workout_index: i8,
    workouts: Vec<String>,
    current_page: Page,
    workout_selection: Option<String>,
    workout_input: Option<String>,
    can_add: bool,
}

impl AppState {
    fn update(&mut self, message: Message) {
        match message {
            Message::NextWorkout => self.on_next_workout(),
            Message::OpenSettings => self.on_open_settings(),
            Message::CloseSettings => self.on_close_settings(),
            Message::WorkoutSelection(workout_option) => self.on_workout_selection(workout_option),
            Message::WorkoutInput(input_option) => self.on_workout_input(input_option),
            Message::AddWorkout => self.on_add_workout(),
        }
    }

    fn on_next_workout(&mut self) {
        let count = self.workouts.iter().count() as i8;
        if count > 0 {
            self.workout_index = (self.workout_index + 1) % count;
            self.write_workouts_state();
        }
    }

    fn on_open_settings(&mut self) {
        self.current_page = Page::Settings;
    }

    fn on_close_settings(&mut self) {
        self.current_page = Page::Main;
    }

    fn on_workout_selection(&mut self, workout_option: Option<String>) {
        self.workout_selection = if self.workout_selection.eq(&workout_option) {
            None
        } else {
            workout_option
        };
    }

    fn on_workout_input(&mut self, workout_input: Option<String>) {
        self.workout_input = workout_input.clone();
        self.can_add =
            matches!(workout_input, Some(input) if self.workouts.iter().all(|s| !input.eq(s)))
    }

    fn on_add_workout(&mut self) {
        if matches!(self.workout_input.clone(), Some(input) if self.workouts.iter().any(|s| input.eq(s)) || input.is_empty())
        {
            return;
        }

        if let Some(input) = self.workout_input.clone() {
            self.workouts.push(input);
            self.write_workouts_state();
        }
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
            .nth(self.workout_index as usize)
            .unwrap_or(&"<empty>".to_owned())
            .clone();
        let total = self.workouts.iter().count();
        let has_next = total > 1;
        let selected_number = if total == 0 {
            0
        } else {
            self.workout_index + 1
        };

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
            workout_selection: self.workout_selection.clone(),
            workout_input: self.workout_input.clone(),
            can_add: self.can_add,
        }
    }

    fn write_workouts_state(&mut self) {
        let result = write_workouts_state(WorkoutsState {
            index: self.workout_index,
            workouts: self.workouts.clone(),
        });
        
        if let Err(error) = result {
            log_error(error.to_string()).ok();
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    NextWorkout,
    OpenSettings,
    CloseSettings,
    WorkoutSelection(Option<String>),
    WorkoutInput(Option<String>),
    AddWorkout,
}

#[derive(Serialize, Deserialize, Default)]
struct WorkoutsState {
    index: i8,
    workouts: Vec<String>,
}
