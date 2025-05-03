use crate::helper::ContainerExtensions;
use crate::ui::{SPACING_M, SPACING_S, SPACING_X, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::Message::WorkoutSelection;
use crate::{Message, OperationFlags, Workout};
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{
    button, center, horizontal_space, text, text_input, Column, Container, Row, Scrollable,
    Space,
};
use iced::{Border, Color, Element, Length, Padding};

const FOOTER_HEIGHT: f32 = 50.0;

pub struct SettingsViewModel<'a> {
    pub workouts: Vec<Workout>,
    pub workout_selection: Option<Workout>,
    pub workout_input: Option<String>,
    pub operation_flags: &'a OperationFlags,
}

pub fn create_settings_page<'a>(view_model: SettingsViewModel) -> impl Into<Element<'a, Message>> {
    Column::new()
        .push(create_body(
            view_model.workouts,
            view_model.workout_selection,
            view_model.workout_input,
            view_model.operation_flags,
        ))
        .push(create_footer())
}

fn create_body<'a>(
    workouts: Vec<Workout>,
    workout_selection: Option<Workout>,
    workout_input: Option<String>,
    flags: &OperationFlags,
) -> impl Into<Element<'a, Message>> {
    Row::new()
        .push(create_workouts_list(workouts, workout_selection))
        .push(create_button_panel(workout_input, flags))
        .padding(SPACING_S)
        .height(WINDOW_HEIGHT - FOOTER_HEIGHT)
}

fn create_workouts_list<'a>(
    workouts: Vec<Workout>,
    workout_selection: Option<Workout>,
) -> impl Into<Element<'a, Message>> {
    let column = workouts
        .into_iter()
        .fold(
            Column::new(),
            |column: Column<'a, Message>, workout: Workout| {
                let is_selected = workout_selection
                    .clone()
                    .map_or(false, |w| w.id == workout.id);
                let button = button(text(workout.text.clone()))
                    .width(Length::Fill)
                    .style(move |_, _| get_list_item_style(is_selected))
                    .on_press(WorkoutSelection(Some(workout)));
                column.push(button)
            },
        )
        .padding(Padding::ZERO.right(SPACING_X))
        .spacing(2);
    let scrollable = Scrollable::with_direction(column, Direction::Vertical(Scrollbar::default()));
    let container = Container::new(scrollable)
        .width((WINDOW_WIDTH / 2.0) - 2.0 * SPACING_S)
        .height(WINDOW_HEIGHT - FOOTER_HEIGHT - 2.0 * SPACING_S)
        .background(20, 20, 20);

    Container::new(container).padding(Padding::new(SPACING_S))
}

fn create_button_panel<'a>(
    workout_input: Option<String>,
    flags: &OperationFlags,
) -> impl Into<Element<'a, Message>> {
    let input_value = workout_input.clone().map_or("".to_owned(), move |s| s);
    let add_input = text_input("New workout", &input_value)
        .on_input(|s| Message::WorkoutInput(Some(s).filter(|s| !s.is_empty())));
    let add_btn = button(text("Add")).on_press_maybe(
        flags
            .contains(OperationFlags::CanAdd)
            .then_some(Message::AddWorkout),
    );
    let update_btn = button(text("Update")).on_press_maybe(
        flags
            .contains(OperationFlags::CanUpdate)
            .then_some(Message::UpdateWorkout),
    );
    let add_update_row = Row::new().push(add_btn).push(update_btn).spacing(SPACING_S);

    let move_up_btn = button(text("\u{2191}")).on_press_maybe(
        flags
            .contains(OperationFlags::CanMoveUp)
            .then_some(Message::MoveWorkoutUp),
    );
    let move_down_btn = button(text("\u{2193}")).on_press_maybe(
        flags
            .contains(OperationFlags::CanMoveDown)
            .then_some(Message::MoveWorkoutDown),
    );
    let remove_btn = button(text("X")).on_press_maybe(
        flags
            .contains(OperationFlags::CanDelete)
            .then_some(Message::InitiateWorkoutDeletion),
    );
    let edit_row = Row::new()
        .push(move_up_btn)
        .push(move_down_btn)
        .push(Space::with_width(SPACING_M))
        .push(remove_btn)
        .spacing(SPACING_S);
    let clear_btn = button(text("Clear")).on_press_maybe(
        flags
            .contains(OperationFlags::CanClear)
            .then_some(Message::InitiateClearance),
    );

    Column::new()
        .push(add_input)
        .push(add_update_row)
        .push(Space::with_height(SPACING_M))
        .push(edit_row)
        .push(Space::with_height(SPACING_M))
        .push(clear_btn)
        .padding(SPACING_S)
        .spacing(SPACING_S)
}

fn create_footer<'a>() -> impl Into<Element<'a, Message>> {
    let ok_btn = button("Ok").on_press(Message::CloseSettings);
    let row = Row::new().push(horizontal_space()).push(ok_btn);

    center(row)
        .height(FOOTER_HEIGHT)
        .padding(Padding::ZERO.right(SPACING_M))
        .dev_background()
}

fn get_list_item_style(is_selected: bool) -> button::Style {
    let background_color = if is_selected {
        Color {
            a: 0.1,
            ..Color::WHITE
        }
    } else {
        Color::TRANSPARENT
    };

    button::Style {
        background: Some(background_color.into()),
        text_color: Color::WHITE,
        border: Border {
            width: 1.0,
            color: Color::WHITE,
            radius: 1.0.into(),
        },
        ..button::Style::default()
    }
}
