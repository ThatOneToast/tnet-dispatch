pub struct StateValues {
    pub new_project: NewProjectState,
    pub project: ProjectState,
    pub layout: LayoutState,
}

impl StateValues {
    pub fn new() -> Self {
        Self {
            new_project: NewProjectState::default(),
            project: ProjectState::default(),
            layout: LayoutState::default(),
        }
    }
}

#[derive(Default)]
pub struct LayoutState {
    pub horizontal_ratio: f32, // Ratio between left and right panels (0.8 = 80% left, 20% right)
    pub vertical_ratio: f32,   // Ratio between top and bottom panels (0.7 = 70% top, 30% bottom)
    pub is_dragging_horizontal: bool,
    pub is_dragging_vertical: bool,
}

impl LayoutState {
    fn default() -> Self {
        Self {
            horizontal_ratio: 0.8,
            vertical_ratio: 0.7,
            is_dragging_horizontal: false,
            is_dragging_vertical: false,
        }
    }
}

#[derive(Default)]
pub struct ProjectState {
    pub current_project: String,
}

#[derive(Default)]
pub struct NewProjectState {
    pub project_name: String,
    pub validation_error: Option<String>,
}
