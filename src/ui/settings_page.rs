use crate::Message;
use crate::helper::DevBackgroundExt;
use iced::widget::{Column, Row, button, center, horizontal_space, text};
use iced::{Element, Padding};

const FOOTER_HEIGHT: u16 = 50;

pub fn create_settings_page<'a>() -> impl Into<Element<'a, Message>> {
    Column::new().push(create_body()).push(create_footer())
}

fn create_body<'a>() -> impl Into<Element<'a, Message>> {
    Row::new()
        .push(create_workouts_list())
        .push(create_button_panel())
}

fn create_workouts_list<'a>() -> impl Into<Element<'a, Message>> {
    center(text("list")).dev_background()
}

fn create_button_panel<'a>() -> impl Into<Element<'a, Message>> {
    center(text("buttons")).dev_background()
}

fn create_footer<'a>() -> impl Into<Element<'a, Message>> {
    let ok_btn = button("Ok").on_press(Message::CloseSettings);
    let row = Row::new().push(horizontal_space()).push(ok_btn);

    center(row)
        .height(FOOTER_HEIGHT)
        .padding(Padding::ZERO.right(10.0))
        .dev_background()
}
