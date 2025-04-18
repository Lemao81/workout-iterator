use crate::Message;
use crate::helper::is_ui_dev;
use iced::widget::container::Style;
use iced::widget::{Column, Container, Row, button, center, horizontal_space, text};
use iced::{Background, Color, Padding};

pub struct ViewModel {
    pub workout: String,
    pub has_next: bool,
    pub selected_number: i8,
    pub total: usize,
}

pub fn create_view<'a>(view_model: ViewModel) -> Container<'a, Message> {
    Container::new(
        Column::new()
            .push(create_header())
            .push(create_body(view_model.workout, view_model.has_next))
            .push(create_footer(view_model.selected_number, view_model.total)),
    )
}

fn create_header<'a>() -> Container<'a, Message> {
    let settings_btn = button("S");

    let mut container = Container::new(center(
        Row::new().push(horizontal_space()).push(settings_btn),
    ))
    .height(50)
    .padding(Padding::from([5.0, 10.0]));

    if is_ui_dev() {
        container = set_container_background(container, Color::from_rgb8(255, 0, 0));
    }

    container
}

fn create_body<'a>(workout: String, has_next: bool) -> Container<'a, Message> {
    let center_width = 250.0;
    let center_height = 100.0;
    
    let workout_txt = center(text(workout).size(28))
        .width(center_width)
        .height(center_height);

    let next_btn = center(
        button("Next")
            .on_press_maybe(if has_next {
                Some(Message::NextWorkout)
            } else {
                None
            })
            .padding(Padding::from([16.0, 28.0])),
    )
    .width(center_width)
    .height(center_height);

    let mut container = Container::new(center(Column::new().push(workout_txt).push(next_btn)))
        .padding(Padding::from(5.0));

    if is_ui_dev() {
        container = set_container_background(container, Color::from_rgb8(0, 255, 0));
    }

    container
}

fn create_footer<'a>(number: i8, total: usize) -> Container<'a, Message> {
    let mut container = Container::new(center(
        Row::new()
            .padding(Padding::from(5.0))
            .push(text(format!("{} from {}", number, total)))
            .push(horizontal_space()),
    ))
    .padding(Padding::from([5.0, 10.0]))
    .height(40);

    if is_ui_dev() {
        container = set_container_background(container, Color::from_rgb8(0, 0, 255));
    }

    container
}

fn set_container_background(container: Container<Message>, color: Color) -> Container<Message> {
    container.style(move |_| Style::default().background(Background::Color(color)))
}
