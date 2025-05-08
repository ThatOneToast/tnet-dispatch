use iced::widget::{button, column, container, horizontal_rule, row, text, text_input, scrollable};
use iced::{Color, Element, Fill, Length};

use crate::{Dispatcher, Message};

use super::active_project;
use super::resizable_split::{horizontal, vertical};

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

pub fn select_existing_project(state: &Dispatcher) -> Element<Message> {
    let title = text("Select Existing Project").size(24);
    
    let description = text("Choose a project from the list:").size(16);
    
    // Create the dropdown header that shows the currently selected project or a placeholder
    let dropdown_text = match &state.states.existing_project.selected_project {
        Some(project) => project.clone(),
        None => "Select a project...".to_string(),
    };
    
    let dropdown_header = button(
        row![
            text(dropdown_text).width(Length::Fill),
            text(if state.states.existing_project.is_dropdown_open { "▲" } else { "▼" })
        ].spacing(10).width(Length::Fill)
    )
    .padding(10)
    .width(Length::Fill)
    .on_press(Message::ToggleProjectDropdown);
    
    // Create the dropdown list
    let mut dropdown_list = column![];
    
    if state.states.existing_project.is_dropdown_open {
        for project in &state.states.existing_project.available_projects {
            let project_row = button(text(project).width(Length::Fill))
                .padding(10)
                .width(Length::Fill)
                .on_press(Message::SelectExistingProject(project.clone()));
            
            dropdown_list = dropdown_list.push(project_row);
        }
    }
    
    // Wrap the dropdown list in a scrollable for better UX with many projects
    let scrollable_dropdown = scrollable(dropdown_list)
        .height(Length::Fixed(
            if state.states.existing_project.is_dropdown_open && !state.states.existing_project.available_projects.is_empty() {
                // Limit height based on number of projects (max 200px)
                (state.states.existing_project.available_projects.len() as f32 * 40.0).min(200.0)
            } else {
                0.0
            }
        ));
    
    // Empty state message when no projects exist
    let empty_message = if state.states.existing_project.available_projects.is_empty() {
        text("No existing projects found. Create a new project first.")
            .size(14)
            .color(Color::from_rgb(0.7, 0.7, 0.7))
    } else {
        text("").height(0.0)
    };
    
    // Action buttons
    let cancel_button = button(text("Cancel"))
        .padding(10)
        .on_press(Message::CancelProjectSelection);
    
    let open_button = if state.states.existing_project.selected_project.is_none() {
        button(text("Open")).padding(10)
    } else {
        button(text("Open"))
            .padding(10)
            .on_press(Message::ConfirmSelectedProject)
    };
    
    let button_row = row![cancel_button, open_button]
        .spacing(10)
        .width(Length::Fill)
        .padding(10);
    
    // Main container
    container(
        column![
            title,
            description,
            dropdown_header,
            scrollable_dropdown,
            empty_message,
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
    use iced::Background;
    
    // Get the layout ratios from state
    let h_ratio = state.states.layout.horizontal_ratio;
    let v_ratio = state.states.layout.vertical_ratio;

    // Main view panel with header
    let main_panel = container(
        column![
            container(text("Main View").size(16))
                .padding(10)
                .width(Length::Fill)
                .style(|_: &_| container::Style {
                    text_color: Some(Color::WHITE),
                    background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.3))),
                    ..Default::default()
                }),
            container(active_project::main_view_panel(state))
                .padding(5)
                .width(Length::Fill)
                .height(Length::Fill)
        ]
        .spacing(1)
    )
    .style(|_: &_| container::Style {
        background: Some(Background::Color(Color::from_rgb(0.18, 0.18, 0.23))),
        border: iced::Border {
            color: Color::from_rgb(0.3, 0.3, 0.4),
            width: 1.0,
            radius: 3.0.into(),
        },
        ..Default::default()
    })
    .width(Length::Fill)
    .height(Length::Fill);

    // Inspector panel with header
    let inspector_panel = container(
        column![
            container(text("Inspector").size(16))
                .padding(10)
                .width(Length::Fill)
                .style(|_: &_| container::Style {
                    text_color: Some(Color::WHITE),
                    background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.3))),
                    ..Default::default()
                }),
            container(active_project::inspector_panel(state))
                .padding(5)
                .width(Length::Fill)
                .height(Length::Fill)
        ]
        .spacing(1)
    )
    .style(|_: &_| container::Style {
        background: Some(Background::Color(Color::from_rgb(0.18, 0.18, 0.23))),
        border: iced::Border {
            color: Color::from_rgb(0.3, 0.3, 0.4),
            width: 1.0,
            radius: 3.0.into(),
        },
        ..Default::default()
    })
    .width(Length::Fill)
    .height(Length::Fill);

    // File tree panel with header
    let file_tree_panel = container(
        column![
            container(text("Files").size(16))
                .padding(10)
                .width(Length::Fill)
                .style(|_: &_| container::Style {
                    text_color: Some(Color::WHITE),
                    background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.3))),
                    ..Default::default()
                }),
            container(active_project::file_tree_panel(state))
                .padding(5)
                .width(Length::Fill)
                .height(Length::Fill)
        ]
        .spacing(1)
    )
    .style(|_: &_| container::Style {
        background: Some(Background::Color(Color::from_rgb(0.18, 0.18, 0.23))),
        border: iced::Border {
            color: Color::from_rgb(0.3, 0.3, 0.4),
            width: 1.0,
            radius: 3.0.into(),
        },
        ..Default::default()
    })
    .width(Length::Fill)
    .height(Length::Fill);

    // First, create a vertical split between main panel and file tree (side by side)
    let vertical_split = vertical(
        main_panel,
        file_tree_panel,
        h_ratio,
        Message::ResizeHorizontal
    );

    // Then create a horizontal split between the vertical split and inspector (top/bottom)
    let layout = horizontal(
        vertical_split,
        inspector_panel,
        v_ratio,
        Message::ResizeVertical
    );

    // Main container
    container(layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(8)
        .style(|_: &_| container::Style {
            background: Some(Background::Color(Color::from_rgb(0.15, 0.15, 0.2))),
            ..Default::default()
        })
        .into()
}
