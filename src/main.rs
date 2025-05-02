#![windows_subsystem = "windows"]

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
use bitflags::bitflags;
use iced::{Element, Task};
use std::cmp::max;
use uuid::Uuid;

fn main() -> iced::Result {
    let workouts_state = read_workouts_state();
    let workouts: Vec<_> = workouts_state
        .workouts
        .into_iter()
        .map(|s| Workout::new(s))
        .collect();
    let mut app_state = AppState {
        workout_index: workouts_state.index,
        workouts: workouts.clone(),
        current_page: Page::Main,
        show_confirmation: None,
        workout_selection: None,
        workout_input: None,
        operation_flags: OperationFlags::empty(),
    };
    app_state
        .operation_flags
        .set(OperationFlags::CanClear, workouts.iter().count() > 0);

    iced::application("Workout Iterator", AppState::update, AppState::view)
        .window_size((WINDOW_WIDTH, WINDOW_HEIGHT))
        .resizable(false)
        .run_with(|| (app_state, Task::none()))
}

bitflags! {
    struct OperationFlags: u8 {
        const CanAdd = 1;
        const CanUpdate = 1 << 1;
        const CanDelete = 1 << 2;
        const CanClear = 1 << 3;
        const CanMoveUp = 1 << 4;
        const CanMoveDown = 1 << 5;
    }
}

struct AppState {
    workout_index: i8,
    workouts: Vec<Workout>,
    current_page: Page,
    show_confirmation: Option<ConfirmationTopic>,
    workout_selection: Option<Workout>,
    workout_input: Option<String>,
    operation_flags: OperationFlags,
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
            Message::UpdateWorkout => self.on_update_workout(),
            Message::InitiateWorkoutDeletion => self.on_initiate_workout_deletion(),
            Message::InitiateClearance => self.on_initiate_clearance(),
            Message::MoveWorkoutUp => self.on_move_workout_up(),
            Message::MoveWorkoutDown => self.on_move_workout_down(),
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
        self.reset_input();
        self.update_operation_flags();
    }

    fn on_close_confirmation_dialog(&mut self, payload: ConfirmationPayload) {
        self.show_confirmation = None;
        if payload.confirmed {
            match payload.topic {
                ConfirmationTopic::WorkoutDeletion => self.delete_workout(),
                ConfirmationTopic::Clearance => self.clear_workouts(),
            };
        }
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

        if let Some(workout) = self.workout_selection.clone() {
            self.workout_input = Some(workout.text)
        }

        self.update_operation_flags();
    }

    fn on_workout_input(&mut self, workout_input: Option<String>) {
        self.workout_input = workout_input.clone();
        self.update_operation_flags();
    }

    fn on_add_workout(&mut self) {
        let input = match self.get_valid_input() {
            None => return,
            Some(s) => s,
        };

        self.workouts.push(Workout::new(input));
        self.update_operation_flags();
        self.write_workouts_state();
    }

    fn on_update_workout(&mut self) {
        let input = match self.get_valid_input() {
            None => return,
            Some(s) => s,
        };

        let workout = match self.workout_selection.clone() {
            None => return,
            Some(w) => w,
        };

        if let Some(position) = self.get_position(workout) {
            self.workouts[position].text = input;
            self.update_operation_flags();
            self.write_workouts_state();
        }
    }

    fn on_initiate_workout_deletion(&mut self) {
        self.show_confirmation = Some(ConfirmationTopic::WorkoutDeletion);
    }

    fn on_initiate_clearance(&mut self) {
        self.show_confirmation = Some(ConfirmationTopic::Clearance);
    }

    fn on_move_workout_up(&mut self) {
        let workout = match self.workout_selection.clone() {
            None => return,
            Some(w) => w,
        };

        let position = match self.get_position(workout.clone()) {
            None => return,
            Some(p) if p <= 0 => return,
            Some(p) => p,
        };

        let removed = self.workouts.remove(position);
        self.workouts.insert(position - 1, removed);
        self.update_operation_flags();
        self.write_workouts_state();
    }

    fn on_move_workout_down(&mut self) {
        let workout = match self.workout_selection.clone() {
            None => return,
            Some(w) => w,
        };

        let position = match self.get_position(workout.clone()) {
            None => return,
            Some(p) if p >= self.workouts.iter().count() - 1 => return,
            Some(p) => p,
        };

        let removed = self.workouts.remove(position);
        self.workouts.insert(position + 1, removed);
        self.update_operation_flags();
        self.write_workouts_state();
    }

    fn delete_workout(&mut self) {
        let workout = match self.workout_selection.clone() {
            None => return,
            Some(w) => w,
        };

        if let Some(position) = self.get_position(workout) {
            self.workouts.remove(position);
            if position <= self.workout_index as usize {
                self.workout_index = max(self.workout_index - 1, 0);
            }

            self.reset_input();
            self.update_operation_flags();
            self.write_workouts_state();
        }
    }

    fn clear_workouts(&mut self) {
        self.workouts.clear();
        self.workout_index = 0;
        self.reset_input();
        self.update_operation_flags();
        self.write_workouts_state();
    }

    fn has_unique_input(&self) -> bool {
        matches!(self.workout_input.clone(), Some(input) if self.workouts.iter().all(|s| !input.eq(&s.text)))
    }

    fn get_valid_input(&mut self) -> Option<String> {
        self.workout_input
            .clone()
            .filter(|s| !s.is_empty() && !self.workouts.iter().any(|w| w.text.eq(s)))
    }

    fn reset_input(&mut self) {
        self.workout_selection = None;
        self.workout_input = None;
    }

    fn update_operation_flags(&mut self) {
        self.operation_flags
            .set(OperationFlags::CanAdd, self.has_unique_input());
        self.operation_flags.set(
            OperationFlags::CanUpdate,
            self.workout_selection.is_some() && self.has_unique_input(),
        );
        self.operation_flags
            .set(OperationFlags::CanDelete, self.workout_selection.is_some());
        self.operation_flags
            .set(OperationFlags::CanClear, self.workouts.iter().count() > 0);
        self.operation_flags.set(
            OperationFlags::CanMoveUp,
            self.workout_selection
                .clone()
                .and_then(|w| self.get_position(w))
                .map_or(false, |p| p > 0),
        );
        self.operation_flags.set(
            OperationFlags::CanMoveDown,
            self.workout_selection
                .clone()
                .and_then(|w| self.get_position(w))
                .map_or(false, |p| p < self.workouts.iter().count() - 1),
        );
    }

    fn get_position(&self, workout: Workout) -> Option<usize> {
        self.workouts.iter().position(|w| w.id == workout.id)
    }

    fn view(&self) -> Element<Message> {
        let page = match self.current_page {
            Page::Main => create_main_page(self.create_main_view_model()).into(),
            Page::Settings => create_settings_page(self.create_settings_view_model()).into(),
        };

        if let Some(topic) = self.show_confirmation.clone() {
            let message = match topic {
                ConfirmationTopic::WorkoutDeletion => None,
                ConfirmationTopic::Clearance => {
                    Some("Removing all workouts. Are you sure?".to_owned())
                }
            };
            let payload = ConfirmationPayload::new(topic, message);
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
            operation_flags: &self.operation_flags,
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
    UpdateWorkout,
    InitiateWorkoutDeletion,
    InitiateClearance,
    MoveWorkoutUp,
    MoveWorkoutDown,
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
