use iced::{
    Element,
    widget::{button, column, container, row},
};

use crate::{Dispatcher, Message};

pub mod project;
pub mod active_project;
pub mod resizable_panel;
pub mod resizable_split;

pub fn on_boarding(_state: &Dispatcher) -> Element<Message> {
    container(column![
        row![
            "Welcome to Tnet-Dispatcher First onboarding dialog",
            " Please select a project."
        ],
        button("CONTINUE").on_press(Message::ContinueOnboarding)
    ])
    .padding(20)
    .into()
}

pub fn on_boarding_2(_state: &Dispatcher) -> Element<Message> {
    container(column![
        row![
            "Welcome to Tnet-Dispatcher Second onboarding dialog",
            " Please select a project.",
            " This is the 2nd onboarding screen."
        ],
        button("FINISH").on_press(Message::NoSelectedProject)
    ])
    .padding(20)
    .into()
}

pub fn no_project_selected(state: &Dispatcher) -> Element<Message> {
    project::no_open_project(state)
}

pub fn creating_project(state: &Dispatcher) -> Element<Message> {
    project::create_new_project_dialog(state)
}

pub fn project_selected(state: &Dispatcher) -> Element<Message> {
    project::opened_project(state)
}

pub fn selecting_existing_project(state: &Dispatcher) -> Element<Message> {
    project::select_existing_project(state)
}
