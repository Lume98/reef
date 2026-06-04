use super::spec::{IslandRevealSpec, IslandWidgetSpec};
use super::{
    cards_builder::build_cards_from_input, compact_bar_builder::build_compact_bar_from_input,
    mascot_builder::build_mascot_from_input, IslandWidgetContentInput,
};
use crate::IslandWidget;

pub(crate) fn build_island_widget_spec(input: &IslandWidgetContentInput) -> IslandWidgetSpec {
    let compact_bar = build_compact_bar_from_input(input);
    let chrome = compact_bar.chrome;

    IslandWidgetSpec {
        mode: input.mode,
        layout: input.layout,
        compact_bar,
        expanded_shell: crate::island::ExpandedShell::new(),
        cards: build_cards_from_input(input),
        mascot: build_mascot_from_input(input),
        glow: None,
        shoulder_left: None,
        shoulder_right: None,
        chrome,
        reveal: IslandRevealSpec::default(),
    }
}

pub fn build_island_widget(input: &IslandWidgetContentInput) -> IslandWidget {
    IslandWidget::from_spec(build_island_widget_spec(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::island_widget::{DisplayMode, IslandSessionInput};
    use crate::ChromeVisibility;

    fn empty_input() -> IslandWidgetContentInput {
        IslandWidgetContentInput::default()
    }

    #[test]
    fn island_widget_spec_maps_expanded_settings_state() {
        let mut input = empty_input();
        input.mode = DisplayMode::Expanded;
        input.settings_active = true;
        let default_layout = input.layout;

        let spec = build_island_widget_spec(&input);
        assert_eq!(spec.mode, DisplayMode::Expanded);
        assert_eq!(spec.layout, default_layout);
        assert_eq!(spec.chrome, ChromeVisibility::expanded());
        assert!(spec.compact_bar.show_actions);
        assert_eq!(spec.cards.len(), 1);
        assert!(spec.mascot.is_some());
        assert!(spec.glow.is_none());
    }

    #[test]
    fn island_widget_spec_maps_status_state() {
        let mut input = empty_input();
        input.mode = DisplayMode::Compact;
        input.active_session_count = 2;
        input.total_session_count = 5;
        input.sessions = vec![IslandSessionInput {
            status: "Running".into(),
            source: "Claude".into(),
            model: Some("claude".into()),
            last_user_prompt: Some("ping".into()),
            last_assistant_message: Some("pong".into()),
            current_tool: Some("bash".into()),
            tool_description: Some("run command".into()),
        }];

        let spec = build_island_widget_spec(&input);
        assert_eq!(spec.cards.len(), 1);
        assert_eq!(spec.compact_bar.headline, "Reef");
        assert_eq!(spec.compact_bar.active_count, "2");
        assert_eq!(spec.compact_bar.total_count, "5");
        assert!(spec.mascot.is_none());
    }

    #[test]
    fn island_widget_spec_reuses_public_fragment_builders() {
        let mut input = empty_input();
        input.mode = DisplayMode::Expanded;
        input.active_session_count = 1;
        input.total_session_count = 2;

        let spec = build_island_widget_spec(&input);
        let compact_bar = build_compact_bar_from_input(&input);
        let cards = build_cards_from_input(&input);
        let mascot = build_mascot_from_input(&input);

        assert_eq!(spec.compact_bar.headline, compact_bar.headline);
        assert_eq!(spec.compact_bar.active_count, compact_bar.active_count);
        assert_eq!(spec.compact_bar.total_count, compact_bar.total_count);
        assert_eq!(spec.compact_bar.show_actions, compact_bar.show_actions);
        assert_eq!(spec.compact_bar.chrome, compact_bar.chrome);
        assert_eq!(spec.cards.len(), cards.len());
        assert_eq!(spec.cards[0].title, cards[0].title);
        assert_eq!(spec.cards[0].style, cards[0].style);
        assert_eq!(spec.mascot.is_some(), mascot.is_some());
    }
}
