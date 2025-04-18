use crate::Message;
use crate::helper::is_ui_dev;
use iced::widget::container::Style;
use iced::widget::{Column, Container, Row, button, center, text};
use iced::{Background, Color, Padding};

pub fn create_view<'a>(workouts: &Vec<String>, index: i8) -> Container<'a, Message> {
    Container::new(
        Column::new()
            .push(create_header())
            .push(create_body(workouts, index))
            .push(create_footer()),
    )
}

fn create_header<'a>() -> Container<'a, Message> {
    let settings_btn = button("S");

    let mut container = Container::new(Row::new().push(settings_btn)).height(50);

    if is_ui_dev() {
        container = set_container_background(container, Color::from_rgb8(255, 0, 0));
    }

    container
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

    let mut container = Container::new(center(Column::new().push(workout_txt).push(next_btn)));

    if is_ui_dev() {
        container = set_container_background(container, Color::from_rgb8(0, 255, 0));
    }

    container
}

fn create_footer<'a>() -> Container<'a, Message> {
    let mut container = Container::new(Row::new()).height(40);

    if is_ui_dev() {
        container = set_container_background(container, Color::from_rgb8(0, 0, 255));
    }

    container
}

fn set_container_background(container: Container<Message>, color: Color) -> Container<Message> {
    container.style(move |_| Style::default().background(Background::Color(color)))
}
