use crate::Message;
use iced::widget::{Stack, center, container, mouse_area, opaque, stack};
use iced::{Color, Element};

pub fn is_ui_dev() -> bool {
    if let Some(value) = option_env!("UI_DEV") {
        return value.to_lowercase() == "true";
    }

    false
}

pub fn modal<'a>(
    base_content: impl Into<Element<'a, Message>>,
    modal_content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    Stack::new()
        .push(base_content)
        .push(opaque(mouse_area(center(opaque(modal_content)).style(
            |_| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            },
        ))))
        .into()
}
