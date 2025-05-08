use iced::{
    Background, Border, Color, Element,
    widget::{Container, container},
};

use crate::app::Message;

pub fn with_border<'a>(
    content: impl Into<Element<'a, Message>>, 
    border_color: Color,
    bg_color: Option<Color>
) -> Container<'a, Message> {
    container(content).style(move |_| iced::widget::container::Style {
        border: Border {
            color: border_color,
            width: 1.5,
            radius: 3.0.into(),
        },
        background: bg_color.map(Background::Color),
        ..Default::default()
    })
    .padding(5)
}

pub fn main_panel<'a>(content: impl Into<Element<'a, Message>>) -> Container<'a, Message> {
    with_border(
        content,
        Color::from_rgba(0.4, 0.4, 0.4, 0.8), // Darker gray with high opacity
        Some(Color::from_rgba(0.98, 0.98, 0.98, 0.3)) // Very light background
    )
}

pub fn inspector_panel<'a>(content: impl Into<Element<'a, Message>>) -> Container<'a, Message> {
    with_border(
        content,
        Color::from_rgba(0.3, 0.3, 0.5, 0.8), // Blue-gray with high opacity
        Some(Color::from_rgba(0.95, 0.95, 1.0, 0.2)) // Very light blue background
    )
}

pub fn file_tree_panel<'a>(content: impl Into<Element<'a, Message>>) -> Container<'a, Message> {
    with_border(
        content,
        Color::from_rgba(0.3, 0.5, 0.3, 0.8), // Green-gray with high opacity
        Some(Color::from_rgba(0.95, 1.0, 0.95, 0.2)) // Very light green background
    )
}
