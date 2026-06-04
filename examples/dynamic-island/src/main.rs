use reef_core::geometry::Size;
use reef_native_panel_core::{DynamicIslandSource, DynamicIslandViewState};
use reef_view::create_root;
use reef_widgets::prelude::{
    Card, CardStyle, CompactBar, DisplayMode, DynamicIsland, MascotPose, MascotWidget,
};

#[derive(Clone, Debug, PartialEq, Eq)]
enum DemoAction {
    Dismiss,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum DemoEffect {
    Close,
}

#[derive(Clone, Copy, Debug, Default)]
struct DemoSource;

impl DynamicIslandSource for DemoSource {
    type Action = DemoAction;
    type Effect = DemoEffect;

    fn build(&self, state: DynamicIslandViewState) -> DynamicIsland<Self::Action> {
        let mode = if state.panel_expanded {
            DisplayMode::Expanded
        } else {
            DisplayMode::Compact
        };
        let mut island = DynamicIsland::new()
            .mode(mode)
            .child(
                CompactBar::new()
                    .headline("Reef")
                    .counts("2", "5")
                    .show_actions(state.panel_expanded),
            )
            .child(
                Card::new(CardStyle::Default)
                    .title(if state.settings_active {
                        "Settings"
                    } else {
                        "Native Panel"
                    })
                    .subtitle(if state.settings_active {
                        "Source-backed demo"
                    } else {
                        "Single-file example"
                    })
                    .action_hint("Swipe to dismiss")
                    .height(96.0),
            )
            .child(
                Card::new(CardStyle::PendingQuestion)
                    .title("Question queue")
                    .subtitle("2 pending")
                    .height(88.0),
            )
            .on_swipe(DemoAction::Dismiss);

        if !state.settings_active {
            island = island.child(MascotWidget::new(200.0, 24.0, 14.0).pose(MascotPose::Running));
        }

        island
    }

    fn resolve_effect(
        &self,
        action: Self::Action,
        _state: DynamicIslandViewState,
    ) -> Option<Self::Effect> {
        match action {
            DemoAction::Dismiss => Some(DemoEffect::Close),
        }
    }
}

fn main() -> Result<(), String> {
    let source = DemoSource;
    let state = DynamicIslandViewState::default();
    let island = source.build(state);
    let widget = island.to_widget();
    let mut root = create_root(Size {
        width: widget.width.max(1.0),
        height: widget.expanded_height.max(widget.compact_height).max(1.0),
    });
    let plan = root.render(island);

    println!(
        "initial-plan hidden={} primitives={} layout={:.0}x{:.0} mode={:?}",
        plan.hidden,
        plan.primitives.len(),
        widget.width,
        widget.expanded_height.max(widget.compact_height),
        widget.mode
    );

    reef_native_panel_windows::run_dynamic_island_standalone(source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef_widgets::prelude::DynamicIslandGesture;

    #[test]
    fn demo_source_builds_visible_island() {
        let widget = DemoSource
            .build(DynamicIslandViewState::default())
            .to_widget();

        assert_ne!(widget.mode, DisplayMode::Hidden);
    }

    #[test]
    fn initial_plan_smoke_test() {
        let source = DemoSource;
        let state = DynamicIslandViewState::default();
        let island = source.build(state);
        let widget = island.to_widget();
        let mut root = create_root(Size {
            width: widget.width.max(1.0),
            height: widget.expanded_height.max(widget.compact_height).max(1.0),
        });
        let plan = root.render(island);

        assert!(!plan.hidden);
        assert!(!plan.primitives.is_empty());
        assert_eq!(
            source.resolve_effect(DemoAction::Dismiss, state),
            Some(DemoEffect::Close)
        );
        assert_eq!(
            source
                .build(state)
                .action_for_gesture(DynamicIslandGesture::Swipe),
            Some(&DemoAction::Dismiss)
        );
    }
}
