use crate::Message;
use iced::Padding;
use iced::widget::{Column, Container, Row, button, center, text};

pub fn create_view<'a>(workouts: &Vec<String>, index: i8) -> Container<'a, Message> {
    Container::new(
        Column::new()
            .push(create_header())
            .push(create_body(workouts, index)),
    )
}

fn create_header<'a>() -> Container<'a, Message> {
    let settings_btn = button("S");

    Container::new(Row::new().push(settings_btn))
}

fn create_body<'a>(workouts: &Vec<String>, index: i8) -> Container<'a, Message> {
    let center_width = 250.0;
    let center_height = 100.0;

    let workout = workouts
        .iter()
        .nth(index as usize)
        .unwrap_or(&"<empty>".to_owned())
        .clone();
    let workout_txt = center(text(workout).size(28))
        .width(center_width)
        .height(center_height);

    let next_btn = center(
        button("Next")
            .on_press_maybe(if workouts.iter().count() > 0 {
                Some(Message::NextWorkout)
            } else {
                None
            })
            .padding(Padding::from([16.0, 28.0])),
    )
    .width(center_width)
    .height(center_height);

    Container::new(center(Column::new().push(workout_txt).push(next_btn)))
}
