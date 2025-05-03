pub mod confirmation_dialog;
pub mod settings_page;

use crate::helper::ContainerExtensions;
use crate::Message;
use iced::widget::{button, center, horizontal_space, text, Column, Container, Row};
use iced::{Element, Padding};

pub const WINDOW_WIDTH: f32 = 500.0;
pub const WINDOW_HEIGHT: f32 = 300.0;
const HEADER_HEIGHT: f32 = 50.0;
const FOOTER_HEIGHT: f32 = 40.0;
const SPACING_S: f32 = 5.0;
const SPACING_M: f32 = 10.0;
const SPACING_X: f32 = 15.0;
const SPACING_XL: f32 = 20.0;
const SPACING_XXL: f32 = 30.0;

pub enum Page {
    Main,
    Settings,
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
        .padding(Padding::ZERO.right(SPACING_M))
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
            .padding(Padding::from([SPACING_X, SPACING_XXL])),
    );

    Container::new(Column::new().push(text).push(button))
        .padding(Padding::ZERO.top(SPACING_XL).bottom(SPACING_XL))
        .dev_background()
}

fn create_footer<'a>(number: i8, total: usize) -> impl Into<Element<'a, Message>> {
    let text = text(format!("{} from {}", number, total));
    let row = Row::new().push(text).push(horizontal_space());

    center(row)
        .height(FOOTER_HEIGHT)
        .padding(Padding::ZERO.left(SPACING_M))
        .dev_background()
}
