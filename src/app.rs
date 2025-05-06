use std::path::PathBuf;

use crate::{DispatchError, config::Config, states::StateValues};

pub enum View {
    Onboarding1,
    ProjectSelected,
    NoProjectSelected,
    CreatingProject,
}

#[derive(Clone, Debug)]
pub enum Message {
    ContinueOnboarding,
    SelectedProject,
    NoSelectedProject,
    CreateNewProject,
    OpenExistingProject,
    NewProjectNameChanged(String),
    ConfirmNewProject,
    CancelNewProject,
    // Panel resize messages
    ResizeHorizontal(f32),  // For horizontal split between left and right
    ResizeVertical(f32),    // For vertical split between top and bottom
}

pub struct Dispatcher {
    pub conf: Config,
    pub cwd: PathBuf,
    pub states: StateValues,
    pub view: View,
}

impl Default for Dispatcher {
    fn default() -> Self {
        let cwd = std::env::current_dir().expect("Failed to get current directory");

        Self {
            conf: Config::default(),
            states: StateValues::new(),
            cwd,
            view: View::NoProjectSelected,
        }
    }
}
