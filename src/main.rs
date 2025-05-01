mod helper;
mod persistence;
mod ui;

use crate::helper::modal;
use crate::persistence::{WorkoutsState, log_error, read_workouts_state, write_workouts_state};
use crate::ui::confirmation_dialog::{
    ConfirmationPayload, ConfirmationTopic, create_confirmation_dialog,
};
use crate::ui::settings_page::{SettingsViewModel, create_settings_page};
use crate::ui::{MainViewModel, Page, WINDOW_HEIGHT, WINDOW_WIDTH, create_main_page};
use iced::{Element, Task};
use uuid::Uuid;

fn main() -> iced::Result {
    let workouts_state = read_workouts_state();
    let workouts = workouts_state
        .workouts
        .into_iter()
        .map(|s| Workout::new(s))
        .collect();
    let app_state = AppState {
        workout_index: workouts_state.index,
        workouts,
        current_page: Page::Main,
        show_workout_deletion_confirmation: false,
        workout_selection: None,
        workout_input: None,
        can_add: false,
        can_delete: false,
    };

    iced::application("Workout Iterator", AppState::update, AppState::view)
        .window_size((WINDOW_WIDTH, WINDOW_HEIGHT))
        .resizable(false)
        .run_with(|| (app_state, Task::none()))
}

struct AppState {
    workout_index: i8,
    workouts: Vec<Workout>,
    current_page: Page,
    show_workout_deletion_confirmation: bool,
    workout_selection: Option<Workout>,
    workout_input: Option<String>,
    can_add: bool,
    can_delete: bool,
}

impl AppState {
    fn update(&mut self, message: Message) {
        match message {
            Message::NextWorkout => self.on_next_workout(),
            Message::OpenSettings => self.on_open_settings(),
            Message::CloseSettings => self.on_close_settings(),
            Message::CloseConfirmationDialog(payload) => self.on_close_confirmation_dialog(payload),
            Message::WorkoutSelection(workout_option) => self.on_workout_selection(workout_option),
            Message::WorkoutInput(input_option) => self.on_workout_input(input_option),
            Message::AddWorkout => self.on_add_workout(),
            Message::InitiateWorkoutDeletion => self.on_initiate_workout_deletion(),
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

    fn on_close_confirmation_dialog(&mut self, payload: ConfirmationPayload) {
        match payload.topic {
            ConfirmationTopic::WorkoutDeletion => {
                self.show_workout_deletion_confirmation = false;
                if payload.confirmed {
                    self.delete_workout();
                }
            }
        };
    }

    fn on_workout_selection(&mut self, workout_option: Option<Workout>) {
        if let (Some(selected), Some(select)) =
            (self.workout_selection.clone(), workout_option.clone())
        {
            self.workout_selection = if selected.id == select.id {
                None
            } else {
                workout_option
            };
        } else {
            self.workout_selection = workout_option;
        }

        self.can_add = self.workout_selection.is_none();
        self.can_delete = self.workout_selection.is_some();
    }

    fn on_workout_input(&mut self, workout_input: Option<String>) {
        self.workout_input = workout_input.clone();
        self.can_add =
            matches!(workout_input, Some(input) if self.workouts.iter().all(|s| !input.eq(&s.text)))
    }

    fn on_add_workout(&mut self) {
        let input = match self.workout_input.clone() {
            None => return,
            Some(s) if s.is_empty() => return,
            Some(s) if self.workouts.iter().any(|w| w.text.eq(&s)) => return,
            Some(s) => s,
        };

        self.workouts.push(Workout::new(input));
        self.write_workouts_state();
    }

    fn on_initiate_workout_deletion(&mut self) {
        self.show_workout_deletion_confirmation = true;
    }

    fn delete_workout(&mut self) {}

    fn view(&self) -> Element<Message> {
        let page = match self.current_page {
            Page::Main => create_main_page(self.create_main_view_model()).into(),
            Page::Settings => create_settings_page(self.create_settings_view_model()).into(),
        };

        if self.show_workout_deletion_confirmation {
            let payload = ConfirmationPayload::new(ConfirmationTopic::WorkoutDeletion);
            modal(
                page,
                create_confirmation_dialog(payload.clone()),
                Message::CloseConfirmationDialog(payload),
            )
        } else {
            page
        }
    }

    fn create_main_view_model(&self) -> MainViewModel {
        let workout = self
            .workouts
            .iter()
            .nth(self.workout_index as usize)
            .map_or("<empty>".to_owned(), |w| w.text.clone());
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
            can_delete: self.can_delete,
        }
    }

    fn write_workouts_state(&mut self) {
        let result = write_workouts_state(WorkoutsState {
            index: self.workout_index,
            workouts: self.workouts.iter().map(|w| w.text.clone()).collect(),
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
    CloseConfirmationDialog(ConfirmationPayload),
    WorkoutSelection(Option<Workout>),
    WorkoutInput(Option<String>),
    AddWorkout,
    InitiateWorkoutDeletion,
}

#[derive(Debug, Clone)]
struct Workout {
    id: Uuid,
    text: String,
}

impl Workout {
    fn new(text: String) -> Workout {
        Workout {
            id: Uuid::new_v4(),
            text,
        }
    }
}
