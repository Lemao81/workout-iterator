use crate::Message;
use crate::Message::WorkoutSelection;
use crate::helper::DevBackgroundExt;
use iced::widget::button::Style;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{Column, Row, Scrollable, button, center, horizontal_space, text};
use iced::{Border, Color, Element, Length, Padding};

const FOOTER_HEIGHT: u16 = 50;

pub struct SettingsViewModel {
    pub workouts: Vec<String>,
    pub workout_selection: Option<String>,
}

pub fn create_settings_page<'a>(view_model: SettingsViewModel) -> impl Into<Element<'a, Message>> {
    Column::new()
        .push(create_body(
            view_model.workouts,
            view_model.workout_selection,
        ))
        .push(create_footer())
}

fn create_body<'a>(
    workouts: Vec<String>,
    workout_selection: Option<String>,
) -> impl Into<Element<'a, Message>> {
    Row::new()
        .push(create_workouts_list(workouts, workout_selection))
        .push(create_button_panel())
}

fn create_workouts_list<'a>(
    workouts: Vec<String>,
    workout_selection: Option<String>,
) -> impl Into<Element<'a, Message>> {
    let column = workouts
        .into_iter()
        .fold(
            Column::new(),
            |column: Column<'a, Message>, workout: String| {
                let is_selected = workout_selection.clone().map_or(false, |w| w == workout);
                let button = button(text(workout.clone()))
                    .width(Length::Fill)
                    .style(move |_, _| get_list_item_style(is_selected))
                    .on_press(WorkoutSelection(Some(workout)));
                column.push(button)
            },
        )
        .padding(Padding::new(5.0).right(15.0))
        .spacing(2);

    Scrollable::with_direction(column, Direction::Vertical(Scrollbar::default()))
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

fn get_list_item_style(is_selected: bool) -> Style {
    let background_color = if is_selected {
        Color {
            a: 0.1,
            ..Color::WHITE
        }
    } else {
        Color::TRANSPARENT
    };

    Style {
        background: Some(background_color.into()),
        text_color: Color::WHITE,
        border: Border {
            width: 1.0,
            color: Color::WHITE,
            radius: 1.0.into(),
        },
        ..Style::default()
    }
}
