use crate::ui::SPACING_XL;
use crate::Message;
use iced::alignment::Horizontal;
use iced::widget::container::Style;
use iced::widget::{button, center, text, Column, Row, Space};
use iced::{Border, Color, Element};

const DIALOG_WIDTH: f32 = 230.0;
const DIALOG_HEIGHT: f32 = 130.0;

#[derive(Debug, Clone)]
pub enum ConfirmationTopic {
    WorkoutDeletion,
}

#[derive(Debug, Clone)]
pub struct ConfirmationPayload {
    pub message: Option<String>,
    pub topic: ConfirmationTopic,
    pub confirmed: bool,
}

impl ConfirmationPayload {
    pub fn new(topic: ConfirmationTopic) -> ConfirmationPayload {
        ConfirmationPayload {
            topic,
            message: None,
            confirmed: false,
        }
    }
}

pub fn create_confirmation_dialog<'a>(
    payload: ConfirmationPayload,
) -> impl Into<Element<'a, Message>> {
    let message = text(
        payload
            .message
            .clone()
            .unwrap_or("Are you sure?".to_owned()),
    );
    let ok_btn =
        button(text("Ok")).on_press(Message::CloseConfirmationDialog(ConfirmationPayload {
            confirmed: true,
            ..payload.clone()
        }));
    let cancel_btn = button(text("Cancel")).on_press(Message::CloseConfirmationDialog(payload));

    let button_row = Row::new().push(ok_btn).push(cancel_btn).spacing(SPACING_XL);

    center(
        Column::new()
            .push(message)
            .push(Space::with_height(SPACING_XL))
            .push(button_row)
            .align_x(Horizontal::Center),
    )
    .width(DIALOG_WIDTH)
    .height(DIALOG_HEIGHT)
    .style(|_| Style {
        background: Some(Color::BLACK.into()),
        border: Border {
            color: Color::from_rgb8(130, 130, 130),
            width: 2.0,
            radius: 5.0.into(),
        },
        ..Style::default()
    })
}
