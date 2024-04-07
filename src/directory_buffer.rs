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
            .saturating_sub(ScrollState::PREVIEW_CUSHION + 1)
            .min(total.saturating_sub(ScrollState::PREVIEW_CUSHION + 1));

        let new_skipped_rows = if !vimlike_scrolling {
            height * (self.current_focus / height.max(1))
        } else if last_focus == None {
            // Just entered the directory
            0
        } else if current_focus == 0 {
            // When focus goes to first node
            0
        } else if current_focus == total.saturating_sub(1) {
            // When focus goes to last node
            total.saturating_sub(height)
        } else if (start_cushion_row..=end_cushion_row).contains(&current_focus) {
            // If within cushioned area; do nothing
            first_visible_row
        } else if current_focus > last_focus.unwrap() {
            // When scrolling down outside the view port
            if current_focus > total.saturating_sub(ScrollState::PREVIEW_CUSHION + 1) {
                // When focusing the last nodes; always view the full last page
                total.saturating_sub(height)
            } else {
                // When scrolling down the view port without reaching the last nodes
                current_focus.saturating_sub(height - 1 - ScrollState::PREVIEW_CUSHION)
            }
        } else if current_focus < last_focus.unwrap() {
            // When scrolling up outside the view port
            if current_focus < ScrollState::PREVIEW_CUSHION {
                // When focusing the first nodes; always view the full first page
                0
            } else if current_focus > end_cushion_row {
                // When scrolling up around from the last rows; do nothing
                first_visible_row
            } else {
                // When scrolling up the view port without reaching the first nodes
                current_focus.saturating_sub(ScrollState::PREVIEW_CUSHION)
            }
        } else {
            // If nothing matches; do nothing
            first_visible_row
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_skipped_rows_non_vimlike_scrolling() {
        let mut state = ScrollState {
            current_focus: 10,
            last_focus: Some(8),
            skipped_rows: 0,
        };

        let height = 5;
        let total = 20;
        let vimlike_scrolling = false;

        let result = state.calc_skipped_rows(height, total, vimlike_scrolling);
        assert_eq!(result, height * (state.current_focus / height.max(1)));
    }

    #[test]
    fn test_calc_skipped_rows_entered_directory() {
        let mut state = ScrollState {
            current_focus: 10,
            last_focus: None,
            skipped_rows: 0,
        };

        let height = 5;
        let total = 20;
        let vimlike_scrolling = true;

        let result = state.calc_skipped_rows(height, total, vimlike_scrolling);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_calc_skipped_rows_top_of_directory() {
        let mut state = ScrollState {
            current_focus: 0,
            last_focus: Some(8),
            skipped_rows: 5,
        };

        let height = 5;
        let total = 20;
        let vimlike_scrolling = true;

        let result = state.calc_skipped_rows(height, total, vimlike_scrolling);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_calc_skipped_rows_bottom_of_directory() {
        let mut state = ScrollState {
            current_focus: 19,
            last_focus: Some(18),
            skipped_rows: 15,
        };

        let height = 5;
        let total = 20;
        let vimlike_scrolling = true;

        let result = state.calc_skipped_rows(height, total, vimlike_scrolling);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_calc_skipped_rows_scrolling_down() {
        let mut state = ScrollState {
            current_focus: 12,
            last_focus: Some(10),
            skipped_rows: 10,
        };

        let height = 5;
        let total = 20;
        let vimlike_scrolling = true;

        let result = state.calc_skipped_rows(height, total, vimlike_scrolling);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_calc_skipped_rows_scrolling_up() {
        let mut state = ScrollState {
            current_focus: 8,
            last_focus: Some(10),
            skipped_rows: 10,
        };

        let height = 5;
        let total = 20;
        let vimlike_scrolling = true;

        let result = state.calc_skipped_rows(height, total, vimlike_scrolling);
        assert_eq!(result, 5);
    }

    // Add more tests for other scenarios...
}
