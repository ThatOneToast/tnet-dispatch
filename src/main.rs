use app::{Dispatcher, Message, View};
use iced::{Element, Theme};
use thiserror::Error;

pub mod app;
pub mod config;
pub mod states;
pub mod views;

#[derive(Debug, Clone, Error)]
pub enum DispatchError {}

pub fn main() -> iced::Result {
    iced::application("Tnet Dispatcher", update, view)
        .theme(theme)
        .run()
}

fn theme(_state: &Dispatcher) -> Theme {
    Theme::CatppuccinMocha
}

fn view(state: &Dispatcher) -> Element<Message> {
    if state.conf.first_time_use {
        println!("First time onboarding");
        views::on_boarding(state)
    } else {
        println!("view state was changed");

        match &state.view {
            View::Onboarding1 => views::on_boarding_2(state),
            View::ProjectSelected => views::project_selected(state),
            View::NoProjectSelected => views::no_project_selected(state),
            View::CreatingProject => views::creating_project(state),
            View::SelectingExistingProject => views::selecting_existing_project(state),
        }
    }
}

fn update(app: &mut Dispatcher, message: Message) {
    match message {
        Message::ContinueOnboarding => {
            app.conf.first_time_use = false;
            // app.conf.save().expect("Failed to save config");
            app.view = View::Onboarding1;
        }
        Message::SelectedProject => {
            app.view = View::ProjectSelected;
        }
        Message::NoSelectedProject => {
            app.view = View::NoProjectSelected;
        }
        Message::CreateNewProject => {
            app.view = View::CreatingProject;
        }
        Message::OpenExistingProject => {
            app.view = View::SelectingExistingProject;
            app.states.existing_project.available_projects = app.conf.get_available_projects();
        }
        Message::NewProjectNameChanged(name) => {
            app.states.new_project.project_name = name.clone();

            // Validate project name: no spaces or special characters allowed
            if name.contains(' ') {
                app.states.new_project.validation_error =
                    Some("Project name cannot contain spaces".to_string());
            } else if !name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
            {
                app.states.new_project.validation_error = Some(
                    "Project name can only contain letters, numbers, underscores and hyphens"
                        .to_string(),
                );
            } else if name.is_empty() {
                app.states.new_project.validation_error =
                    Some("Project name cannot be empty".to_string());
            } else {
                app.states.new_project.validation_error = None;
            }
        }
        Message::CancelNewProject => {
            app.states.new_project.project_name = String::new();
            app.states.new_project.validation_error = None;
            app.view = View::NoProjectSelected;
        }
        Message::ConfirmNewProject => {
            if app.states.new_project.validation_error.is_none()
                && !app.states.new_project.project_name.is_empty()
            {
                app.states.project.current_project = app.states.new_project.project_name.clone();
                
                std::fs::create_dir_all(
                    app.conf
                        .get_data_path()
                        .join(&app.states.project.current_project),
                )
                .expect("Failed to create project directory");
                app.states.project.current_project_path = app
                    .conf
                    .get_data_path()
                    .join(&app.states.project.current_project);

                app.states.new_project.project_name = String::new();
                app.states.new_project.validation_error = None;
                app.view = View::ProjectSelected;
            }
        }
        Message::LoadExistingProjects => {
            app.states.existing_project.available_projects = app.conf.get_available_projects();
        }
        Message::ExistingProjectsLoaded(projects) => {
            app.states.existing_project.available_projects = projects;
        }
        Message::SelectExistingProject(project_name) => {
            app.states.existing_project.selected_project = Some(project_name);
        }
        Message::ToggleProjectDropdown => {
            app.states.existing_project.is_dropdown_open = !app.states.existing_project.is_dropdown_open;
        }
        Message::ConfirmSelectedProject => {
            if let Some(project_name) = &app.states.existing_project.selected_project {
                app.states.project.current_project = project_name.clone();
                app.states.project.current_project_path = app.conf.get_data_path().join(project_name);
                app.view = View::ProjectSelected;
            }
        }
        Message::CancelProjectSelection => {
            app.states.existing_project.selected_project = None;
            app.states.existing_project.is_dropdown_open = false;
            app.view = View::NoProjectSelected;
        }
        // Handle panel resize messages
        Message::ResizeHorizontal(ratio) => {
            // Update the horizontal ratio (clamped between 0.2 and 0.9)
            app.states.layout.horizontal_ratio = ratio.max(0.2).min(0.9);
        }
        Message::ResizeVertical(ratio) => {
            // Update the vertical ratio (clamped between 0.2 and 0.9)
            app.states.layout.vertical_ratio = ratio.max(0.2).min(0.9);
        }
    }
}
