#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod helper;
mod persistence;
mod ui;

use crate::helper::modal;
use crate::persistence::{
    log_error, read_window_state, read_workouts_state, write_window_state, write_workouts_state, Position,
    WorkoutsState,
};
use crate::ui::confirmation_dialog::{
    create_confirmation_dialog, ConfirmationPayload, ConfirmationTopic,
};
use crate::ui::settings_page::{create_settings_page, SettingsViewModel};
use crate::ui::{create_main_page, MainViewModel, Page, WINDOW_HEIGHT, WINDOW_WIDTH};
use bitflags::bitflags;
use iced::window::{Id, Settings};
use iced::Event::Window;
use iced::{event, window, Element, Point, Size, Subscription, Task};
use image::ImageFormat;
use std::cmp::max;
use uuid::Uuid;

const ICON_BYTES: &[u8] = include_bytes!("../resources/icon.ico");

fn main() -> iced::Result {
    let workouts_state = read_workouts_state();
    let workouts: Vec<_> = workouts_state
        .workouts
        .into_iter()
        .map(|s| Workout::new(s))
        .collect();
    let window_position = read_window_state();
    let mut app_state = AppState {
        workout_index: workouts_state.index,
        workouts: workouts.clone(),
        window_position: window_position.clone(),
        ..AppState::default()
    };
    app_state
        .operation_flags
        .set(OperationFlags::CanClear, workouts.iter().count() > 0);

    iced::application("Workout Iterator", AppState::update, AppState::view)
        .window(Settings {
            size: Size::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            position: window_position.map_or(window::Position::Default, |p| {
                window::Position::Specific(Point::from([p.x, p.y]))
            }),
            icon: window::icon::from_file_data(ICON_BYTES, Some(ImageFormat::Ico)).ok(),
            exit_on_close_request: false,
            ..Settings::default()
        })
        .subscription(AppState::window_subscription)
        .resizable(false)
        .run_with(|| (app_state, window::get_latest().map(Message::WindowId)))
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
    window_id: Option<Id>,
    workout_index: i8,
    workouts: Vec<Workout>,
    current_page: Page,
    show_confirmation: Option<ConfirmationTopic>,
    workout_selection: Option<Workout>,
    workout_input: Option<String>,
    operation_flags: OperationFlags,
    window_position: Option<Position>,
}

impl Default for AppState {
    fn default() -> AppState {
        AppState {
            window_id: None,
            workout_index: 0,
            workouts: vec![],
            current_page: Page::Main,
            show_confirmation: None,
            workout_selection: None,
            workout_input: None,
            operation_flags: OperationFlags::empty(),
            window_position: None,
        }
    }
}

impl AppState {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowId(id_option) => self.on_window_id(id_option),
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
            Message::WindowMoved(x, y) => self.on_window_moved(x, y),
            Message::WindowCloseRequest => self.on_window_close_request(),
        }
    }

    fn on_window_id(&mut self, id: Option<Id>) -> Task<Message> {
        self.window_id = id;

        Task::none()
    }

    fn on_next_workout(&mut self) -> Task<Message> {
        let count = self.workouts.iter().count() as i8;
        if count > 0 {
            self.workout_index = (self.workout_index + 1) % count;
            self.write_workouts_state();
        }

        Task::none()
    }

    fn on_open_settings(&mut self) -> Task<Message> {
        self.current_page = Page::Settings;

        Task::none()
    }

    fn on_close_settings(&mut self) -> Task<Message> {
        self.current_page = Page::Main;
        self.reset_input();
        self.update_operation_flags();

        Task::none()
    }

    fn on_close_confirmation_dialog(&mut self, payload: ConfirmationPayload) -> Task<Message> {
        self.show_confirmation = None;
        if payload.confirmed {
            match payload.topic {
                ConfirmationTopic::WorkoutDeletion => self.delete_workout(),
                ConfirmationTopic::Clearance => self.clear_workouts(),
            };
        }

        Task::none()
    }

    fn on_workout_selection(&mut self, workout_option: Option<Workout>) -> Task<Message> {
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

        Task::none()
    }

    fn on_workout_input(&mut self, workout_input: Option<String>) -> Task<Message> {
        self.workout_input = workout_input.clone();
        self.update_operation_flags();

        Task::none()
    }

    fn on_add_workout(&mut self) -> Task<Message> {
        let input = match self.get_valid_input() {
            None => return Task::none(),
            Some(s) => s,
        };

        self.workouts.push(Workout::new(input));
        self.workout_input = None;
        self.update_operation_flags();
        self.write_workouts_state();

        Task::none()
    }

    fn on_update_workout(&mut self) -> Task<Message> {
        let input = match self.get_valid_input() {
            None => return Task::none(),
            Some(s) => s,
        };

        let workout = match self.workout_selection.clone() {
            None => return Task::none(),
            Some(w) => w,
        };

        if let Some(position) = self.get_position(workout) {
            self.workouts[position].text = input;
            self.update_operation_flags();
            self.write_workouts_state();
        }

        Task::none()
    }

    fn on_initiate_workout_deletion(&mut self) -> Task<Message> {
        self.show_confirmation = Some(ConfirmationTopic::WorkoutDeletion);

        Task::none()
    }

    fn on_initiate_clearance(&mut self) -> Task<Message> {
        self.show_confirmation = Some(ConfirmationTopic::Clearance);

        Task::none()
    }

    fn on_move_workout_up(&mut self) -> Task<Message> {
        let workout = match self.workout_selection.clone() {
            None => return Task::none(),
            Some(w) => w,
        };

        let position = match self.get_position(workout.clone()) {
            None => return Task::none(),
            Some(p) if p <= 0 => return Task::none(),
            Some(p) => p,
        };

        let removed = self.workouts.remove(position);
        self.workouts.insert(position - 1, removed);
        self.update_operation_flags();
        self.write_workouts_state();

        Task::none()
    }

    fn on_move_workout_down(&mut self) -> Task<Message> {
        let workout = match self.workout_selection.clone() {
            None => return Task::none(),
            Some(w) => w,
        };

        let position = match self.get_position(workout.clone()) {
            None => return Task::none(),
            Some(p) if p >= self.workouts.iter().count() - 1 => return Task::none(),
            Some(p) => p,
        };

        let removed = self.workouts.remove(position);
        self.workouts.insert(position + 1, removed);
        self.update_operation_flags();
        self.write_workouts_state();

        Task::none()
    }

    fn on_window_moved(&mut self, x: f32, y: f32) -> Task<Message> {
        self.window_position = Some(Position::new(x, y));

        Task::none()
    }

    fn on_window_close_request(&self) -> Task<Message> {
        if let Some(position) = self.window_position.clone() {
            let result = write_window_state(position);
            if let Err(error) = result {
                log_error(error.to_string()).ok();
            }
        }

        match self.window_id {
            None => std::process::exit(0),
            Some(window_id) => window::close(window_id),
        }
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

    fn window_subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _, _| match event {
            Window(window::Event::Moved(p)) => Some(Message::WindowMoved(p.x, p.y)),
            Window(window::Event::CloseRequested) => Some(Message::WindowCloseRequest),
            _ => None,
        })
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
    WindowId(Option<Id>),
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
    WindowMoved(f32, f32),
    WindowCloseRequest,
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

#[cfg(test)]
mod tests {
    use crate::{AppState, Workout};

    #[test]
    fn test_has_unique_input_given_unique_input_should_be_true() {
        let state = AppState {
            workouts: vec![
                Workout::new(String::from("workout1")),
                Workout::new(String::from("workout2")),
            ],
            workout_input: Some(String::from("workout3")),
            ..AppState::default()
        };
        assert!(state.has_unique_input())
    }

    #[test]
    fn test_has_unique_input_given_existing_input_should_be_false() {
        let state = AppState {
            workouts: vec![
                Workout::new(String::from("workout1")),
                Workout::new(String::from("workout2")),
            ],
            workout_input: Some(String::from("workout2")),
            ..AppState::default()
        };
        assert!(!state.has_unique_input())
    }
}
