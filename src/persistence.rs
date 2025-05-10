use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Error, Write};

const WORKOUTS_JSON: &'static str = "workouts.json";
const WINDOW_JSON: &'static str = "window.json";
const ERROR_LOG: &'static str = "error.log";

#[derive(Serialize, Deserialize, Default)]
pub struct WorkoutsState {
    pub index: i8,
    pub workouts: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct WindowState {
    pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position { x, y }
    }
}

pub fn read_workouts_state() -> WorkoutsState {
    if let Err(error) = maybe_create_initial_workouts_json() {
        println!("{}", error);
        std::process::exit(1);
    }

    let workouts_state = match read_workouts_json() {
        Err(error) => {
            println!("{}", error);
            std::process::exit(2);
        }
        Ok(s) => s,
    };

    if let Err(error) = validate_workouts_state(&workouts_state) {
        println!("{}", error);
        std::process::exit(3);
    }

    workouts_state
}

pub fn write_workouts_state(workouts_state: WorkoutsState) -> Result<(), Error> {
    let mut file = File::create(WORKOUTS_JSON)?;
    let buffer = serde_json::to_vec(&workouts_state)?;
    file.write_all(&buffer)?;

    Ok(())
}

pub fn read_window_state() -> Option<Position> {
    match fs::exists(WINDOW_JSON) {
        Err(error) => {
            println!("{}", error);

            return None;
        }
        Ok(exists) if !exists => {
            return None;
        }
        _ => (),
    }

    let window_state = match read_window_json() {
        Err(error) => {
            println!("{}", error);

            return None;
        }
        Ok(state) => state,
    };

    match validate_window_state(&window_state) {
        Err(error) => {
            println!("{}", error);

            None
        }
        Ok(_) => Some(window_state.position),
    }
}

pub fn write_window_state(position: Position) -> Result<(), Error> {
    let mut file = File::create(WINDOW_JSON)?;
    let buffer = serde_json::to_vec(&WindowState { position })?;
    file.write_all(&buffer)?;

    Ok(())
}

pub fn log_error(error: impl AsRef<str>) -> Result<(), Error> {
    println!("{}", error.as_ref());

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(ERROR_LOG)?;
    writeln!(file, "{}  -  {}", timestamp, error.as_ref())?;

    Ok(())
}

fn maybe_create_initial_workouts_json() -> Result<(), Error> {
    if fs::exists(WORKOUTS_JSON)? {
        return Ok(());
    }

    write_workouts_state(WorkoutsState::default())?;

    Ok(())
}

fn read_workouts_json() -> Result<WorkoutsState, Error> {
    let buffer = fs::read(WORKOUTS_JSON)?;

    Ok(serde_json::from_slice(&buffer)?)
}

fn read_window_json() -> Result<WindowState, Error> {
    let buffer = fs::read(WINDOW_JSON)?;

    Ok(serde_json::from_slice(&buffer)?)
}

fn validate_workouts_state(workouts_state: &WorkoutsState) -> Result<(), &'static str> {
    let count = workouts_state.workouts.iter().count() as i8;
    match workouts_state.index {
        i if i < 0 || (count == 0 && i != 0) || (count > 0 && i >= count) => {
            Err("invalid workouts.json: index out of range")
        }
        _ => Ok(()),
    }
}

fn validate_window_state(window_state: &WindowState) -> Result<(), &'static str> {
    match window_state.position {
        Position { x, y } if x < 0.0 || y < 0.0 => Err("invalid window.json: negative position(s)"),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use crate::persistence::{
        Position, WindowState, WorkoutsState, validate_window_state, validate_workouts_state,
    };

    #[test]
    fn test_validate_workouts_state_given_default_should_be_ok() {
        let state = WorkoutsState::default();

        assert!(validate_workouts_state(&state).is_ok())
    }

    #[test]
    fn test_validate_workouts_state_given_index_in_range_should_be_ok() {
        let state = WorkoutsState {
            workouts: vec![String::from("workout1"), String::from("workout2")],
            index: 1,
        };

        assert!(validate_workouts_state(&state).is_ok())
    }

    #[test]
    fn test_validate_workouts_state_given_index_not_in_range_should_be_err() {
        let state = WorkoutsState {
            workouts: vec![String::from("workout1"), String::from("workout2")],
            index: 2,
        };

        assert!(validate_workouts_state(&state).is_err())
    }

    #[test]
    fn test_validate_workouts_state_given_negative_index_should_be_err() {
        let state = WorkoutsState {
            workouts: vec![String::from("workout1"), String::from("workout2")],
            index: -1,
        };

        assert!(validate_workouts_state(&state).is_err())
    }

    #[test]
    fn test_validate_workouts_state_given_empty_list_index_not_zero_should_be_err() {
        let state = WorkoutsState {
            workouts: vec![],
            index: 1,
        };

        assert!(validate_workouts_state(&state).is_err())
    }

    #[test]
    fn test_validate_window_state_given_default_should_be_ok() {
        let state = WindowState::default();

        assert!(validate_window_state(&state).is_ok())
    }

    #[test]
    fn test_validate_window_state_given_valid_position_should_be_ok() {
        let state = WindowState {
            position: Position::new(1.0, 1.0),
        };

        assert!(validate_window_state(&state).is_ok())
    }

    #[test]
    fn test_validate_window_state_given_negative_x_should_be_err() {
        let state = WindowState {
            position: Position::new(-1.0, 1.0),
        };

        assert!(validate_window_state(&state).is_err())
    }

    #[test]
    fn test_validate_window_state_given_negative_y_should_be_err() {
        let state = WindowState {
            position: Position::new(1.0, -1.0),
        };

        assert!(validate_window_state(&state).is_err())
    }
}
