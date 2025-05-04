use crate::Message;
use iced::widget::{Container, Stack, center, container, mouse_area, opaque};
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
            }))
            .on_press(close_message),
        ))
        .into()
}

pub trait ContainerExtensions<'a> {
    fn background(self, r: u8, g: u8, b: u8) -> Self;
    fn dev_background(self) -> Self;
}

impl ContainerExtensions<'_> for Container<'_, Message> {
    fn background(self, r: u8, g: u8, b: u8) -> Self {
        self.style(move |_| container::Style {
            background: Some(Color::from_rgb8(r, g, b).into()),
            ..container::Style::default()
        })
    }

    fn dev_background(self) -> Self {
        if !is_ui_dev() {
            return self;
        }

        self.style(move |_| {
            let mut rng = rand::rng();
            container::Style::default().background(Background::Color(Color::from_rgba8(
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
