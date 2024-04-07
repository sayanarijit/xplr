use crate::node::Node;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScrollState {
    current_focus: usize,
    pub last_focus: Option<usize>,
    pub skipped_rows: usize,
}

impl ScrollState {
    /* The number of visible next lines when scrolling towards either ends of the view port */
    pub const PREVIEW_CUSHION: usize = 3;

    pub fn set_focus(&mut self, current_focus: usize) {
        self.last_focus = Some(self.current_focus);
        self.current_focus = current_focus;
    }

    pub fn get_focus(&self) -> usize {
        self.current_focus
    }

    pub fn calc_skipped_rows(
        &mut self,
        height: usize,
        total: usize,
        vimlike_scrolling: bool,
    ) -> usize {
        let current_focus = self.current_focus;
        let last_focus = self.last_focus;
        let first_visible_row = self.skipped_rows;

        // Calculate the cushion rows at the start and end of the view port
        let start_cushion_row = first_visible_row + ScrollState::PREVIEW_CUSHION;
        let end_cushion_row = (first_visible_row + height)
            .saturating_sub(ScrollState::PREVIEW_CUSHION + 1);

        let new_skipped_rows = if !vimlike_scrolling {
            height * (self.current_focus / height.max(1))
        } else if last_focus == None {
            // Just entered the directory
            0
        } else if current_focus == 0 {
            // Focus on first node
            0
        } else if current_focus == total.saturating_sub(1) {
            // Focus on last node
            total.saturating_sub(height)
        } else if current_focus > last_focus.unwrap() {
            // Scrolling down
            if current_focus <= end_cushion_row {
                first_visible_row
            } else if total <= (current_focus + ScrollState::PREVIEW_CUSHION) {
                first_visible_row
            } else {
                (self.current_focus + ScrollState::PREVIEW_CUSHION + 1)
                    .saturating_sub(height)
            }
        } else {
            // Scrolling up
            if current_focus >= start_cushion_row {
                first_visible_row
            } else if current_focus <= ScrollState::PREVIEW_CUSHION {
                0
            } else {
                current_focus.saturating_sub(ScrollState::PREVIEW_CUSHION)
            }
        };

        new_skipped_rows
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DirectoryBuffer {
    pub parent: String,
    pub nodes: Vec<Node>,
    pub total: usize,
    pub scroll_state: ScrollState,

    #[serde(skip, default = "now")]
    pub explored_at: OffsetDateTime,
}

impl DirectoryBuffer {
    pub fn new(parent: String, nodes: Vec<Node>, current_focus: usize) -> Self {
        let total = nodes.len();
        Self {
            parent,
            nodes,
            total,
            scroll_state: ScrollState {
                current_focus,
                last_focus: None,
                skipped_rows: 0,
            },
            explored_at: now(),
        }
    }

    pub fn focused_node(&self) -> Option<&Node> {
        self.nodes.get(self.scroll_state.current_focus)
    }
}

fn now() -> OffsetDateTime {
    OffsetDateTime::now_local()
        .ok()
        .unwrap_or_else(OffsetDateTime::now_utc)
}
