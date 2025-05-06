use iced::widget::{button, column, container, horizontal_rule, row, text, text_input};
use iced::{Color, Element, Fill, Length};

use crate::{Dispatcher, Message};

use super::active_project;

pub fn no_open_project(_state: &Dispatcher) -> Element<Message> {
    // Title section
    let title = text("Welcome to Tnet-Dispatcher").size(36); // Size in pixels

    let subtitle = text("Select a project to begin working").size(16);

    // Project options
    let new_project = button(row![text("New Project")].spacing(10))
        .width(200)
        .padding(10)
        .on_press(Message::CreateNewProject);

    let open_project = button(row![text("Open Project")].spacing(10))
        .width(200)
        .padding(10)
        .on_press(Message::OpenExistingProject);

    // Container for the buttons
    let actions = row![new_project, open_project].spacing(20).padding(20);

    // Recent projects would go here
    let recent_title = text("Recent Projects").size(20);

    let empty_message = text("No recent projects").size(14);

    // Main content layout
    container(
        column![
            title,
            subtitle,
            horizontal_rule(10),
            actions,
            horizontal_rule(10),
            recent_title,
            empty_message
        ]
        .spacing(20)
        .padding(20),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Fill)
    .center_y(Fill)
    .into()
}

pub fn create_new_project_dialog(state: &Dispatcher) -> Element<Message> {
    let title = text("Create New Project").size(24);

    let description = text("Enter a name for your new project:").size(16);

    let project_name_input = text_input("My Project", &state.states.new_project.project_name)
        .padding(10)
        .width(Length::Fill)
        .on_input(Message::NewProjectNameChanged);

    let validation_message = if let Some(error) = &state.states.new_project.validation_error {
        text(error).size(14).color(Color::from_rgb(0.9, 0.2, 0.2))
    } else {
        // Use a non-breaking space to maintain consistent height without visible text
        text("\u{00A0}").size(14)
    };

    let cancel_button = button(text("Cancel"))
        .padding(10)
        .on_press(Message::CancelNewProject);

    let create_button = if state.states.new_project.validation_error.is_some()
        || state.states.new_project.project_name.is_empty()
    {
        button(text("Create")).padding(10)
    } else {
        button(text("Create"))
            .padding(10)
            .on_press(Message::ConfirmNewProject)
    };

    let button_row = row![cancel_button, create_button]
        .spacing(10)
        .width(Length::Fill)
        .padding(10);

    container(
        column![
            title,
            description,
            project_name_input,
            validation_message,
            button_row
        ]
        .spacing(20)
        .padding(20)
        .width(Length::Fill),
    )
    .width(Length::Fixed(400.00))
    .center_x(Length::Fixed(400.00))
    .center_y(Length::Fixed(400.00))
    .into()
}



pub fn opened_project(state: &Dispatcher) -> Element<Message> {
    // Main content area (70% of height)
    let main_view = container(active_project::main_view_panel(state))
        .width(Length::Fill)
        .height(Length::FillPortion(7)) // 70% 
        .padding(5);
    
    // Bottom panel (30% of height)
    let bottom_panel = container(active_project::inspector_panel(state))
        .width(Length::Fill)
        .height(Length::FillPortion(3)) // 30% 
        .padding(5);
    
    // Left side containing main view and bottom panel
    let left_side = column![main_view, bottom_panel]
        .spacing(5) // Small gap between the panels
        .width(Length::FillPortion(8)) // 80% 
        .height(Length::Fill);
    
    // Right side file tree panel
    let file_tree = container(active_project::file_tree_panel(state))
        .width(Length::FillPortion(2)) // 20% 
        .height(Length::Fill)
        .padding(5);
    
    // Combine left and right sides
    let content = row![left_side, file_tree]
        .spacing(5) 
        .width(Length::Fill)
        .height(Length::Fill);
    
    // Main container
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        .into()
}
