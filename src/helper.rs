use crate::Message;
use iced::widget::container::Style;
use iced::widget::{Container, Stack, center, mouse_area, opaque};
use iced::{Background, Color, Element};
use rand::Rng;

pub fn modal<'a>(
    base_content: impl Into<Element<'a, Message>>,
    modal_content: impl Into<Element<'a, Message>>,
    close_message: Message,
) -> Element<'a, Message> {
    Stack::new()
        .push(base_content)
        .push(opaque(
            mouse_area(center(opaque(modal_content)).style(|_| {
                Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..Style::default()
                }
            }))
            .on_press(close_message),
        ))
        .into()
}

pub trait DevBackgroundExt<'a> {
    fn dev_background(self) -> Self;
}

impl DevBackgroundExt<'_> for Container<'_, Message> {
    fn dev_background(self) -> Self {
        if !is_ui_dev() {
            return self;
        }

        self.style(move |_| {
            let mut rng = rand::rng();
            Style::default().background(Background::Color(Color::from_rgba8(
                rng.random_range(0..=255),
                rng.random_range(0..=255),
                rng.random_range(0..=255),
                0.8,
            )))
        })
    }
}

fn is_ui_dev() -> bool {
    if let Some(value) = option_env!("UI_DEV") {
        return value.to_lowercase() == "true";
    }

    false
}
