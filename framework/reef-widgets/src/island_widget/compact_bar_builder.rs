use crate::{ChromeVisibility, CompactBar, DisplayMode};

use super::IslandWidgetContentInput;

const DEFAULT_HEADLINE: &str = "Reef";

pub(super) fn build_compact_bar(input: &IslandWidgetContentInput) -> CompactBar {
    let mut bar = CompactBar::new();
    bar.headline = DEFAULT_HEADLINE.to_string();
    bar.headline_emphasized = input.mode == DisplayMode::Expanded;
    bar.active_count = input.active_session_count.to_string();
    bar.total_count = input.total_session_count.to_string();
    bar.completion_count = 0;
    bar.show_actions = input.mode == DisplayMode::Expanded || input.settings_active;
    bar.debug_mode = false;
    bar.chrome = if input.mode == DisplayMode::Expanded {
        ChromeVisibility::expanded()
    } else {
        ChromeVisibility::compact()
    };
    bar
}
