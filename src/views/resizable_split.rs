use iced::{
    advanced::{
        layout::{Limits, Node},
        renderer,
        widget::{Operation, Tree},
        Layout, Widget,
    },
    event,
    mouse::{self, Cursor},
    Border, Color, Element, Event, Length, Rectangle, Point, Shadow, Size,
};

pub struct ResizableSplit<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    first: Element<'a, Message, Theme, Renderer>,
    second: Element<'a, Message, Theme, Renderer>,
    is_horizontal: bool,  // true = horizontal split (top/bottom), false = vertical split (left/right)
    ratio: f32,           // position of the split (0.0 - 1.0)
    on_resize: Box<dyn Fn(f32) -> Message + 'a>,
    min_size: (u16, u16), // minimum sizes for first and second elements
}

impl<'a, Message, Theme, Renderer> ResizableSplit<'a, Message, Theme, Renderer> {
    pub fn new(
        first: impl Into<Element<'a, Message, Theme, Renderer>>,
        second: impl Into<Element<'a, Message, Theme, Renderer>>,
        ratio: f32,
        on_resize: impl Fn(f32) -> Message + 'a,
    ) -> Self {
        Self {
            first: first.into(),
            second: second.into(),
            is_horizontal: false, // Vertical split by default
            ratio: ratio.max(0.1).min(0.9), // Constrain ratio
            on_resize: Box::new(on_resize),
            min_size: (50, 50), // Default minimum sizes
        }
    }

    pub fn horizontal(mut self) -> Self {
        self.is_horizontal = true;
        self
    }

    pub fn min_size(mut self, first: u16, second: u16) -> Self {
        self.min_size = (first, second);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DragState {
    Idle,
    Dragging,
}

impl Default for DragState {
    fn default() -> Self {
        DragState::Idle
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for ResizableSplit<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let max_size = limits.max(); 
        
        // Ensure the tree has children
        if tree.children.is_empty() {
            tree.children = vec![
                Tree::new(&self.first),
                Tree::new(&self.second),
            ];
        } else if tree.children.len() == 1 {
            // If there's only one child, add the second
            tree.children.push(Tree::new(&self.second));
        }
        
        let (first_child, second_child) = tree.children.split_at_mut(1);
        let first_child = &mut first_child[0];
        let second_child = &mut second_child[0];
        
        if self.is_horizontal {
            // Horizontal split (first above second)
            let max_width = max_size.width;
            let max_height = max_size.height;
            let first_height = (max_height * self.ratio).floor();
            let second_height = max_height - first_height;
            
            let first_limits = Limits::new(
                Size::new(0.0, self.min_size.0 as f32),
                Size::new(max_width, first_height),
            );
            
            let second_limits = Limits::new(
                Size::new(0.0, self.min_size.1 as f32),
                Size::new(max_width, second_height),
            );
            
            let first_layout = self.first.as_widget().layout(
                first_child,
                renderer, 
                &first_limits
            );
            
            let second_layout = self.second.as_widget().layout(
                second_child,
                renderer, 
                &second_limits
            );
            
            // Position the children
            let first_layout = first_layout.move_to(Point::new(0.0, 0.0));
            let second_layout = second_layout.move_to(Point::new(0.0, first_height));
            
            // Create the layout node
            let node = Node::with_children(
                Size::new(max_width, max_height),
                vec![first_layout, second_layout],
            );
            
            node
        } else {
            // Vertical split (first beside second)
            let max_width = max_size.width;
            let max_height = max_size.height;
            let first_width = (max_width * self.ratio).floor();
            let second_width = max_width - first_width;
            
            let first_limits = Limits::new(
                Size::new(self.min_size.0 as f32, 0.0),
                Size::new(first_width, max_height),
            );
            
            let second_limits = Limits::new(
                Size::new(self.min_size.1 as f32, 0.0),
                Size::new(second_width, max_height),
            );
            
            let first_layout = self.first.as_widget().layout(
                first_child,
                renderer, 
                &first_limits
            );
            
            let second_layout = self.second.as_widget().layout(
                second_child,
                renderer, 
                &second_limits
            );
            
            // Position the children
            let first_layout = first_layout.move_to(Point::new(0.0, 0.0));
            let second_layout = second_layout.move_to(Point::new(first_width, 0.0));
            
            // Create the layout node
            let node = Node::with_children(
                Size::new(max_width, max_height),
                vec![first_layout, second_layout],
            );
            
            node
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        if let Some(cursor_position) = cursor.position() {
            // Define divider bounds with increased thickness for easier interaction
            let divider_thickness = 10.0;
            let children = layout.children().collect::<Vec<_>>();
            
            if let Some(first_layout) = children.first() {
                let first_bounds = first_layout.bounds();
                
                let divider_bounds = if self.is_horizontal {
                    Rectangle {
                        x: layout.bounds().x,
                        y: first_bounds.y + first_bounds.height - divider_thickness / 2.0,
                        width: layout.bounds().width,
                        height: divider_thickness,
                    }
                } else {
                    Rectangle {
                        x: first_bounds.x + first_bounds.width - divider_thickness / 2.0,
                        y: layout.bounds().y,
                        width: divider_thickness,
                        height: layout.bounds().height,
                    }
                };
                
                if divider_bounds.contains(cursor_position) {
                    return if self.is_horizontal {
                        mouse::Interaction::ResizingVertically
                    } else {
                        mouse::Interaction::ResizingHorizontally
                    };
                }
            }
        }
        
        // Otherwise, use the children's interactions
        let mut child_interaction = mouse::Interaction::default();
        
        for (i, (child_tree, child_layout)) in tree.children.iter().zip(layout.children()).enumerate() {
            let interaction = if i == 0 {
                self.first.as_widget().mouse_interaction(
                    child_tree,
                    child_layout,
                    cursor,
                    viewport,
                    renderer,
                )
            } else {
                self.second.as_widget().mouse_interaction(
                    child_tree,
                    child_layout,
                    cursor,
                    viewport,
                    renderer,
                )
            };
            
            child_interaction = interaction.max(child_interaction);
        }
        
        child_interaction
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let mut children = layout.children();
        
        // Draw the first child
        if let Some(first_layout) = children.next() {
            if let Some(first_state) = state.children.get(0) {
                self.first.as_widget().draw(
                    first_state,
                    renderer,
                    theme,
                    style,
                    first_layout,
                    cursor,
                    viewport,
                );
            }
        }
        
        // Draw the second child
        if let Some(second_layout) = children.next() {
            if let Some(second_state) = state.children.get(1) {
                self.second.as_widget().draw(
                    second_state,
                    renderer,
                    theme,
                    style,
                    second_layout,
                    cursor,
                    viewport,
                );
            }
        }
        
        // Draw the resizing handle
        let children = layout.children().collect::<Vec<_>>();
        if let Some(first_layout) = children.first() {
            let first_bounds = first_layout.bounds();
            
            if self.is_horizontal {
                let y = first_bounds.y + first_bounds.height;
                
                let handle_bounds = Rectangle {
                    x: layout.bounds().x,
                    y: y - 2.0,
                    width: layout.bounds().width,
                    height: 5.0,
                };
                
                // Draw a subtle divider line
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: handle_bounds,
                        border: Border {
                            radius: 0.0.into(),
                            width: 0.0,
                            color: Color::TRANSPARENT,
                        },
                        shadow: Shadow::default(),
                    },
                    Color::from_rgba(0.5, 0.5, 0.5, 0.7), // More visible divider
                );
            } else {
                let x = first_bounds.x + first_bounds.width;
                
                let handle_bounds = Rectangle {
                    x: x - 2.0,
                    y: layout.bounds().y,
                    width: 5.0,
                    height: layout.bounds().height,
                };
                
                // Draw a subtle divider line
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: handle_bounds,
                        border: Border {
                            radius: 0.0.into(),
                            width: 0.0,
                            color: Color::TRANSPARENT,
                        },
                        shadow: Shadow::default(),
                    },
                    Color::from_rgba(0.5, 0.5, 0.5, 0.7), // More visible divider
                );
            }
        }
    }
    
    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        // Extract drag state if it exists or create a new one
        let drag_state = match &state.state {
            iced::advanced::widget::tree::State::Some(state_box) => {
                if let Some(state_ref) = state_box.downcast_ref::<DragState>() {
                    *state_ref
                } else {
                    state.state = iced::advanced::widget::tree::State::new(DragState::default());
                    DragState::default()
                }
            },
            iced::advanced::widget::tree::State::None => {
                state.state = iced::advanced::widget::tree::State::new(DragState::default());
                DragState::default()
            }
        };
        
        // Ensure the tree has children
        if state.children.is_empty() {
            state.children = vec![
                Tree::new(&self.first),
                Tree::new(&self.second),
            ];
        } else if state.children.len() == 1 {
            // If there's only one child, add the second
            state.children.push(Tree::new(&self.second));
        }
        
        // Track dragging state
        let is_dragging = matches!(drag_state, DragState::Dragging);
        
        // Handle divider dragging
        if let Event::Mouse(mouse_event) = &event {
            // Define divider bounds
            let divider_thickness = 5.0;
            let children = layout.children().collect::<Vec<_>>();
            
            if let Some(first_layout) = children.first() {
                let first_bounds = first_layout.bounds();
                
                let divider_bounds = if self.is_horizontal {
                    // For horizontal split (top/bottom)
                    Rectangle {
                        x: layout.bounds().x,
                        y: first_bounds.y + first_bounds.height - divider_thickness / 2.0,
                        width: layout.bounds().width,
                        height: divider_thickness,
                    }
                } else {
                    // For vertical split (left/right)
                    Rectangle {
                        x: first_bounds.x + first_bounds.width - divider_thickness / 2.0,
                        y: layout.bounds().y,
                        width: divider_thickness,
                        height: layout.bounds().height,
                    }
                };
                
                match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        if let Some(cursor_position) = cursor.position() {
                            if divider_bounds.contains(cursor_position) {
                                if let iced::advanced::widget::tree::State::Some(state_box) = &mut state.state {
                                    if let Some(state_ref) = state_box.downcast_mut::<DragState>() {
                                        *state_ref = DragState::Dragging;
                                    }
                                }
                                return event::Status::Captured;
                            }
                        }
                    },
                    mouse::Event::CursorMoved { .. } => {
                        if is_dragging {
                            if let Some(cursor_position) = cursor.position() {
                                let bounds = layout.bounds();
                                
                                // Calculate new ratio based on cursor position
                                let new_ratio = if self.is_horizontal {
                                    // For horizontal split (top/bottom), use y coordinate
                                    ((cursor_position.y - bounds.y) / bounds.height)
                                        .max(0.1)  // Ensure minimum size for first component
                                        .min(0.9)  // Ensure minimum size for second component
                                } else {
                                    // For vertical split (left/right), use x coordinate
                                    ((cursor_position.x - bounds.x) / bounds.width)
                                        .max(0.1)  // Ensure minimum size for first component
                                        .min(0.9)  // Ensure minimum size for second component
                                };
                                
                                // Only publish if the ratio changed meaningfully
                                shell.publish((self.on_resize)(new_ratio));
                                return event::Status::Captured;
                            }
                        }
                    },
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        if is_dragging {
                            if let iced::advanced::widget::tree::State::Some(state_box) = &mut state.state {
                                if let Some(state_ref) = state_box.downcast_mut::<DragState>() {
                                    *state_ref = DragState::Idle;
                                }
                            }
                            return event::Status::Captured;
                        }
                    },
                    _ => {}
                }
            }
        }
        
        // Pass events to children
        let mut children = layout.children();
        let first_status = if let Some(first_layout) = children.next() {
            if let Some(first_child) = state.children.get_mut(0) {
                self.first.as_widget_mut().on_event(
                    first_child,
                    event.clone(),
                    first_layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                )
            } else {
                event::Status::Ignored
            }
        } else {
            event::Status::Ignored
        };
        
        if first_status == event::Status::Captured {
            return first_status;
        }
        
        if let Some(second_layout) = children.next() {
            if let Some(second_child) = state.children.get_mut(1) {
                self.second.as_widget_mut().on_event(
                    second_child,
                    event,
                    second_layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                )
            } else {
                event::Status::Ignored
            }
        } else {
            event::Status::Ignored
        }
    }

    fn children(&self) -> Vec<Tree> {
        vec![
            Tree::new(&self.first),
            Tree::new(&self.second),
        ]
    }

    fn diff(&self, tree: &mut Tree) {
        match &tree.state {
            iced::advanced::widget::tree::State::Some(state_box) => {
                if state_box.downcast_ref::<DragState>().is_none() {
                    tree.state = iced::advanced::widget::tree::State::new(DragState::default());
                }
            },
            iced::advanced::widget::tree::State::None => {
                tree.state = iced::advanced::widget::tree::State::new(DragState::default());
            }
        }
        
        // Ensure the tree has children
        if tree.children.is_empty() {
            tree.children = vec![
                Tree::new(&self.first),
                Tree::new(&self.second),
            ];
        } else if tree.children.len() == 1 {
            // If there's only one child, add the second
            tree.children.push(Tree::new(&self.second));
        }
        
        // Now we can safely access the children
        tree.children.get_mut(0).map(|child| self.first.as_widget().diff(child));
        tree.children.get_mut(1).map(|child| self.second.as_widget().diff(child));
    }
    
    fn operate(
        &self,
        state: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            let mut children = layout.children();
            
            if let Some(first_layout) = children.next() {
                if let Some(first_child) = state.children.get_mut(0) {
                    self.first.as_widget().operate(
                        first_child,
                        first_layout,
                        renderer,
                        operation,
                    );
                }
            }
            
            if let Some(second_layout) = children.next() {
                if let Some(second_child) = state.children.get_mut(1) {
                    self.second.as_widget().operate(
                        second_child,
                        second_layout,
                        renderer,
                        operation,
                    );
                }
            }
        });
    }
}

/// Creates a horizontal split (top/bottom) with the given content.
pub fn horizontal<'a, Message, Theme, Renderer>(
    first: impl Into<Element<'a, Message, Theme, Renderer>>,
    second: impl Into<Element<'a, Message, Theme, Renderer>>,
    ratio: f32,
    on_resize: impl Fn(f32) -> Message + 'a,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: 'a + renderer::Renderer,
{
    Element::new(ResizableSplit::new(first, second, ratio, on_resize).horizontal())
}

/// Creates a vertical split (left/right) with the given content.
pub fn vertical<'a, Message, Theme, Renderer>(
    first: impl Into<Element<'a, Message, Theme, Renderer>>,
    second: impl Into<Element<'a, Message, Theme, Renderer>>,
    ratio: f32,
    on_resize: impl Fn(f32) -> Message + 'a,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: 'a + renderer::Renderer,
{
    Element::new(ResizableSplit::new(first, second, ratio, on_resize))
}

impl<'a, Message, Theme, Renderer> From<ResizableSplit<'a, Message, Theme, Renderer>> 
for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: 'a + renderer::Renderer,
{
    fn from(split: ResizableSplit<'a, Message, Theme, Renderer>) -> Self {
        Element::new(split)
    }
}