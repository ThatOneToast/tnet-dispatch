use iced::{widget::{container, column, text}, Length, Fill, Element};

use crate::app::{Dispatcher, Message};

pub fn main_view_panel(_state: &Dispatcher) -> Element<Message> {
    container(
        column![
            text("Main View Area").size(20),
            text("This is the main content area where the primary workspace is displayed")
        ]
        .spacing(10)
        .width(Length::Fill)
        .height(Length::Fill)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Fill)
    .center_y(Fill)
    .padding(10)
    .into()
}

pub fn inspector_panel(_state: &Dispatcher) -> Element<Message> {
    container(
        column![
            text("Inspector Panel").size(20),
            text("Properties and details for selected items appear here")
        ]
        .spacing(10)
        .width(Length::Fill)
        .height(Length::Fill)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Fill)
    .center_y(Fill)
    .padding(10)
    .into()
}

pub fn file_tree_panel(_state: &Dispatcher) -> Element<Message> {
    container(
        column![
            text("File Tree").size(20),
            text("Project structure and files")
        ]
        .spacing(10)
        .width(Length::Fill)
        .height(Length::Fill)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Fill)
    .center_y(Fill)
    .padding(10)
    .into()
}