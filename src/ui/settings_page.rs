use crate::Message;
use iced::Element;
use iced::widget::{Container, Row, text};

pub fn create_settings_page<'a>() -> impl Into<Element<'a, Message>> {
    Container::new(
        Row::new()
            .push(create_workouts_list_view())
            .push(create_button_panel()),
    )
}

fn create_workouts_list_view<'a>() -> impl Into<Element<'a, Message>> {
    text("list")
}

fn create_button_panel<'a>() -> impl Into<Element<'a, Message>> {
    text("buttons")
}
