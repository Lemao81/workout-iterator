use crate::Message;
use iced::widget::{text, Container, Row};
use iced::Element;

pub fn create_settings_page<'a>() -> Element<'a, Message> {
    Container::new(Row::new().push(create_workouts_list_view()).push(create_button_panel())).into()
}

fn create_workouts_list_view<'a>() -> impl Into<Element<'a,Message>> {
    todo!()
}

fn create_button_panel<'a,Theme,Renderer>() -> impl Into<Element<'a,Message, Theme, Renderer>> + Sized {
    todo!()
}
