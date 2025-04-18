mod helper;
mod ui;

use crate::ui::{ViewModel, create_view};
use iced::{Element, Task};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;

const CONFIG_JSON: &'static str = "workouts.json";

fn main() -> iced::Result {
    if let Err(error) = maybe_create_initial_workouts_json() {
        println!("{}", error);
        std::process::exit(1);
    }

    let read_result = read_workouts_json();
    if let Err(error) = read_result {
        println!("{}", error);
        std::process::exit(2);
    }

    let workouts_state = read_result.unwrap();
    if let Err(error) = validate_workouts_state(&workouts_state) {
        println!("{}", error);
        std::process::exit(3);
    }

    let app = App {
        workouts: workouts_state.workouts,
        index: workouts_state.index,
    };

    iced::application("Workout Iterator", App::update, App::view)
        .window_size((500.0, 300.0))
        .resizable(false)
        .run_with(|| (app, Task::none()))
}

fn maybe_create_initial_workouts_json() -> Result<(), std::io::Error> {
    if fs::exists(CONFIG_JSON)? {
        return Ok(());
    }

    let mut file = File::create(CONFIG_JSON)?;
    let buffer = serde_json::to_vec(&WorkoutsState::default())?;
    file.write_all(&buffer)?;

    Ok(())
}

fn read_workouts_json() -> Result<WorkoutsState, std::io::Error> {
    let buffer = fs::read(CONFIG_JSON)?;

    Ok(serde_json::from_slice(&buffer)?)
}

fn validate_workouts_state(workouts_state: &WorkoutsState) -> Result<(), &'static str> {
    let count = workouts_state.workouts.iter().count() as i8;
    match workouts_state.index {
        i if i < 0 || (count > 0 && i >= count) => Err("invalid workouts.json: index out of range"),
        _ => Ok(()),
    }
}

struct App {
    index: i8,
    workouts: Vec<String>,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::NextWorkout => {
                let count = self.workouts.iter().count() as i8;
                if count > 0 {
                    self.index = (self.index + 1) % count;
                }
            }
            _ => return,
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
