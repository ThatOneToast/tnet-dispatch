use iced::{
    Alignment, Element, Length,
    widget::{button, column, container, scrollable, text},
};
use std::path::PathBuf;

use crate::app::{Dispatcher, Message};

pub fn main_view_panel(_state: &Dispatcher) -> Element<Message> {
    container(scrollable(
        column![
            text("Main View Area").size(20),
            text("This is the main content area where the primary workspace is displayed"),
            text("Content line 1"),
            text("Content line 2"),
            text("Content line 3"),
            text("Content line 4"),
            text("Content line 5")
        ]
        .spacing(10)
        .width(Length::Fill)
        .align_x(Alignment::Start)
        .padding([15, 15]),
    ))
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(10)
    .into()
}

pub fn inspector_panel(_state: &Dispatcher) -> Element<Message> {
    container(scrollable(
        column![
            text("Inspector Panel").size(20),
            text("Properties and details for selected items appear here"),
            text("Property 1: Value"),
            text("Property 2: Value"),
            text("Property 3: Value")
        ]
        .spacing(10)
        .width(Length::Fill)
        .align_x(Alignment::Start)
        .padding([15, 15]),
    ))
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(10)
    .into()
}

pub fn file_tree_panel(state: &Dispatcher) -> Element<Message> {
    let proj_dir = state.states.project.current_project_path.clone();
    
    // Structure to hold file tree entry data
    #[derive(Debug)]
    struct TreeEntry {
        path: PathBuf,
        is_directory: bool,
        indent_level: usize,
        is_last_in_level: bool,
    }
    
    // Collect file entries for display
    let mut tree_entries: Vec<TreeEntry> = Vec::new();

    // Helper function to build tree entry data
    fn build_tree_entries(
        dir_path: &std::path::Path,
        base_dir: &std::path::Path,
        indent_level: usize,
        entries: &mut Vec<TreeEntry>,
    ) {
        if let Ok(dir_entries) = std::fs::read_dir(dir_path) {
            // Convert to Vec for easier processing and sort
            let mut dir_entries: Vec<_> = dir_entries.flatten().collect();
            // Sort directories first, then files
            dir_entries.sort_by(|a, b| {
                let a_is_dir = a.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                let b_is_dir = b.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                if a_is_dir && !b_is_dir {
                    std::cmp::Ordering::Less
                } else if !a_is_dir && b_is_dir {
                    std::cmp::Ordering::Greater
                } else {
                    a.file_name().cmp(&b.file_name())
                }
            });
            
            let entry_count = dir_entries.len();
            
            for (idx, entry) in dir_entries.iter().enumerate() {
                let is_last = idx == entry_count - 1;
                let path = entry.path();
                
                if let Ok(file_type) = entry.file_type() {
                    let file_name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown");
                    
                    if file_type.is_dir() {
                        // Add directory entry
                        entries.push(TreeEntry {
                            path: path.clone(),
                            is_directory: true,
                            indent_level,
                            is_last_in_level: is_last,
                        });
                        
                        // Process subdirectory
                        build_tree_entries(
                            &path,
                            base_dir,
                            indent_level + 1,
                            entries,
                        );
                    } else if file_type.is_file() {
                        // Only add files with .json or .proc extensions
                        if file_name.ends_with(".json") || file_name.ends_with(".proc") {
                            entries.push(TreeEntry {
                                path: path.clone(),
                                is_directory: false,
                                indent_level,
                                is_last_in_level: is_last,
                            });
                        }
                    }
                }
            }
        }
    }

    // Build the tree entries starting from the project directory
    build_tree_entries(&proj_dir, &proj_dir, 0, &mut tree_entries);
    
    // Create elements for the file tree
    let mut tree_elements = Vec::new();
    tree_elements.push(text("File Tree").size(20).into());
    tree_elements.push(text("Project structure and files").into());
    
    // Add all tree entries as clickable elements
    for entry in &tree_entries {
        let file_name = entry.path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown");
            
        // Create indentation prefix
        let mut prefix = String::new();
        for i in 0..entry.indent_level {
            // Find if any parent at this level is a last entry
            let is_last_parent = tree_entries.iter()
                .filter(|e| e.indent_level == i && e.is_directory)
                .any(|e| e.is_last_in_level);
                
            if is_last_parent {
                prefix.push_str("    "); // Space for last item branch
            } else {
                prefix.push_str("│   "); // Vertical line for continuing branch
            }
        }
        
        // Add the connector for this entry
        let connector = if entry.is_last_in_level { "└── " } else { "├── " };
        
        // Create the full display text
        let display_text = if entry.is_directory {
            format!("{}{}{}/", prefix, connector, file_name)
        } else {
            format!("{}{}{}", prefix, connector, file_name)
        };
        
        // Create a clickable button for the entry
        let btn = button(text(display_text))
            .padding([2, 5])
            .width(Length::Fill)
            .on_press(Message::SelectedProject);
            
        tree_elements.push(btn.into());
    }
    
    container(scrollable(
        column(tree_elements)
            .spacing(5)
            .width(Length::Fill)
            .align_x(Alignment::Start)
            .padding([15, 15]),
    ))
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(10)
    .into()
}
