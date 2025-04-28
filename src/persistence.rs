use crate::WorkoutsState;
use std::fs;
use std::fs::File;
use std::io::{Error, Write};

const CONFIG_JSON: &'static str = "workouts.json";

pub fn read_workouts_state() -> WorkoutsState {
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

    workouts_state
}

pub fn write_workouts_state(workouts_state: WorkoutsState) -> Result<(), Error> {
    let mut file = File::create(CONFIG_JSON)?;
    let buffer = serde_json::to_vec(&workouts_state)?;
    file.write_all(&buffer)?;

    Ok(())
}

fn maybe_create_initial_workouts_json() -> Result<(), Error> {
    if fs::exists(CONFIG_JSON)? {
        return Ok(());
    }

    write_workouts_state(WorkoutsState::default())?;

    Ok(())
}

fn read_workouts_json() -> Result<WorkoutsState, Error> {
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
