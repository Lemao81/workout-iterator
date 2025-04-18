use crate::ui::create_header;
use iced::widget::{button, center, container, text, Column};
use iced::{Element, Padding, Task};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;

mod ui {
    use crate::Message;
    use iced::widget::{button, Container, Row};

    pub fn create_header<'a>() -> Container<'a, Message> {
        let settings_btn = button("S");

        Container::new(Row::new().push(settings_btn))
    }
}

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
        let center_width = 250.0;
        let center_height = 100.0;

        let workout = self
            .workouts
            .iter()
            .nth(self.index as usize)
            .unwrap_or(&"<empty>".to_owned())
            .clone();
        let workout_txt = center(text(workout).size(28))
            .width(center_width)
            .height(center_height);

        let next_btn = center(
            button("Next")
                .on_press_maybe(if self.workouts.iter().count() > 0 {
                    Some(Message::NextWorkout)
                } else {
                    None
                })
                .padding(Padding::from([16.0, 28.0])),
        )
        .width(center_width)
        .height(center_height);

        let column = Column::with_children(vec![
            create_header().into(),
            workout_txt.into(),
            next_btn.into(),
        ]);

        container(center(column)).into()
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
