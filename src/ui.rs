pub mod settings_page;

use crate::Message;
use crate::helper::DevBackgroundExt;
use iced::widget::{Column, Container, Row, button, center, horizontal_space, text};
use iced::{Element, Padding};

pub const WINDOW_WIDTH: f32 = 500.0;
pub const WINDOW_HEIGHT: f32 = 300.0;
const HEADER_HEIGHT: u16 = 50;
const FOOTER_HEIGHT: u16 = 40;

pub enum Page {
    Main,
    Settings
}

pub struct MainViewModel {
    pub workout: String,
    pub has_next: bool,
    pub selected_number: i8,
    pub total: usize,
}

pub fn create_main_page<'a>(view_model: MainViewModel) -> impl Into<Element<'a, Message>> {
    Column::new()
        .push(create_header())
        .push(create_body(view_model.workout, view_model.has_next))
        .push(create_footer(view_model.selected_number, view_model.total))
}

fn create_header<'a>() -> impl Into<Element<'a, Message>> {
    let settings_btn = button("S").on_press(Message::OpenSettings);
    let row = Row::new().push(horizontal_space()).push(settings_btn);

    center(row)
        .height(HEADER_HEIGHT)
        .padding(Padding::ZERO.right(10.0))
        .dev_background()
}

fn create_body<'a>(workout: String, has_next: bool) -> impl Into<Element<'a, Message>> {
    let text = center(text(workout).size(28));
    let button = center(
        button("Next")
            .on_press_maybe(if has_next {
                Some(Message::NextWorkout)
            } else {
                None
            })
            .padding(Padding::from([16.0, 28.0])),
    );

    Container::new(Column::new().push(text).push(button))
        .padding(Padding::ZERO.top(20.0).bottom(20.0))
        .dev_background()
}

fn create_footer<'a>(number: i8, total: usize) -> impl Into<Element<'a, Message>> {
    let text = text(format!("{} from {}", number, total));
    let row = Row::new().push(text).push(horizontal_space());

    center(row)
        .height(FOOTER_HEIGHT)
        .padding(Padding::ZERO.left(10.0))
        .dev_background()
}
