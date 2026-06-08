use crate::core::geometry::{Rect, Size};
use crate::layout::Constraints;
use crate::view::widget_host::{PaintContext, Widget};

use crate::prelude::{
    Card, CardStyle, ChromeVisibility, CompactBar, CompactShoulder, CompletionGlow, DisplayMode,
    ExpandedShell, IslandRenderOverrides, IslandRevealSpec, IslandWidget, IslandWidgetLayout,
    IslandWidgetSpec, MascotWidget, ProgressBar,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DynamicIslandGesture {
    Click,
    Swipe,
    Hover,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DynamicIslandActionBinding<Action> {
    pub gesture: DynamicIslandGesture,
    pub action: Action,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DynamicIslandTarget {
    Session(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DynamicIslandTargetActionBinding<Action> {
    pub target: DynamicIslandTarget,
    pub gesture: DynamicIslandGesture,
    pub action: Action,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DynamicIslandActions<Action> {
    pub click: Option<Action>,
    pub swipe: Option<Action>,
    pub hover: Option<Action>,
}

impl<Action> Default for DynamicIslandActions<Action> {
    fn default() -> Self {
        Self {
            click: None,
            swipe: None,
            hover: None,
        }
    }
}

#[derive(Clone)]
pub enum DynamicIslandChild {
    CompactBar(CompactBar),
    Card(Card),
    Mascot(MascotWidget),
    ProgressBar(ProgressBar),
}

impl From<CompactBar> for DynamicIslandChild {
    fn from(value: CompactBar) -> Self {
        Self::CompactBar(value)
    }
}

impl From<Card> for DynamicIslandChild {
    fn from(value: Card) -> Self {
        Self::Card(value)
    }
}

impl From<MascotWidget> for DynamicIslandChild {
    fn from(value: MascotWidget) -> Self {
        Self::Mascot(value)
    }
}

impl From<ProgressBar> for DynamicIslandChild {
    fn from(value: ProgressBar) -> Self {
        Self::ProgressBar(value)
    }
}

#[derive(Clone)]
pub struct DynamicIsland<Action> {
    spec: IslandWidgetSpec,
    render_overrides: Option<IslandRenderOverrides>,
    actions: DynamicIslandActions<Action>,
    target_bindings: Vec<DynamicIslandTargetActionBinding<Action>>,
}

impl<Action> DynamicIsland<Action> {
    pub fn new() -> Self {
        Self {
            spec: IslandWidgetSpec::default(),
            render_overrides: None,
            actions: DynamicIslandActions::default(),
            target_bindings: Vec::new(),
        }
    }

    pub fn child(mut self, child: impl Into<DynamicIslandChild>) -> Self {
        match child.into() {
            DynamicIslandChild::CompactBar(bar) => {
                self.spec.chrome = bar.chrome;
                self.spec.layout.compact_height = bar.height;
                self.spec.compact_bar = bar;
            }
            DynamicIslandChild::Card(card) => self.spec.cards.push(card),
            DynamicIslandChild::Mascot(mascot) => self.spec.mascot = Some(mascot),
            DynamicIslandChild::ProgressBar(progress) => {
                self.spec.cards.push(
                    Card::new(CardStyle::Default)
                        .title("Progress")
                        .height(40.0)
                        .action_hint(format!("{:.0}%", progress.value.clamp(0.0, 1.0) * 100.0)),
                );
            }
        }
        self
    }

    pub fn mode(mut self, mode: DisplayMode) -> Self {
        self.spec.mode = mode;
        self
    }

    pub fn layout(mut self, layout: IslandWidgetLayout) -> Self {
        self.spec.layout = layout;
        self
    }

    pub fn chrome(mut self, chrome: ChromeVisibility) -> Self {
        self.spec.chrome = chrome;
        self.spec.compact_bar.chrome = chrome;
        self
    }

    pub fn reveal(mut self, progress: f64, entering: bool) -> Self {
        self.spec.reveal = IslandRevealSpec::new(progress, entering);
        self
    }

    pub fn expanded_shell(mut self, expanded_shell: ExpandedShell) -> Self {
        self.spec.expanded_shell = expanded_shell;
        self
    }

    pub fn glow(mut self, glow: CompletionGlow) -> Self {
        self.spec.glow = Some(glow);
        self
    }

    pub fn shoulder_left(mut self, shoulder: CompactShoulder) -> Self {
        self.spec.shoulder_left = Some(shoulder);
        self
    }

    pub fn shoulder_right(mut self, shoulder: CompactShoulder) -> Self {
        self.spec.shoulder_right = Some(shoulder);
        self
    }

    pub fn render_overrides(mut self, overrides: IslandRenderOverrides) -> Self {
        self.render_overrides = Some(overrides);
        self
    }

    pub fn on_click(mut self, action: Action) -> Self {
        self.actions.click = Some(action);
        self
    }

    pub fn on_swipe(mut self, action: Action) -> Self {
        self.actions.swipe = Some(action);
        self
    }

    pub fn on_hover(mut self, action: Action) -> Self {
        self.actions.hover = Some(action);
        self
    }

    pub fn on_target_click(mut self, target: DynamicIslandTarget, action: Action) -> Self {
        self.target_bindings.push(DynamicIslandTargetActionBinding {
            target,
            gesture: DynamicIslandGesture::Click,
            action,
        });
        self
    }

    pub fn actions(&self) -> &DynamicIslandActions<Action> {
        &self.actions
    }

    pub fn target_bindings(&self) -> &[DynamicIslandTargetActionBinding<Action>] {
        &self.target_bindings
    }

    pub fn action_for_gesture(&self, gesture: DynamicIslandGesture) -> Option<&Action> {
        match gesture {
            DynamicIslandGesture::Click => self.actions.click.as_ref(),
            DynamicIslandGesture::Swipe => self.actions.swipe.as_ref(),
            DynamicIslandGesture::Hover => self.actions.hover.as_ref(),
        }
    }

    pub fn action_for_target_gesture(
        &self,
        target: &DynamicIslandTarget,
        gesture: DynamicIslandGesture,
    ) -> Option<&Action> {
        self.target_bindings
            .iter()
            .rev()
            .find(|binding| &binding.target == target && binding.gesture == gesture)
            .map(|binding| &binding.action)
    }

    pub fn bindings(&self) -> Vec<DynamicIslandActionBinding<&Action>> {
        let mut bindings = Vec::new();
        if let Some(action) = self.actions.click.as_ref() {
            bindings.push(DynamicIslandActionBinding {
                gesture: DynamicIslandGesture::Click,
                action,
            });
        }
        if let Some(action) = self.actions.swipe.as_ref() {
            bindings.push(DynamicIslandActionBinding {
                gesture: DynamicIslandGesture::Swipe,
                action,
            });
        }
        if let Some(action) = self.actions.hover.as_ref() {
            bindings.push(DynamicIslandActionBinding {
                gesture: DynamicIslandGesture::Hover,
                action,
            });
        }
        bindings
    }

    pub fn to_widget(&self) -> IslandWidget {
        let mut widget = IslandWidget::from_spec(self.spec.clone());
        if let Some(overrides) = self.render_overrides {
            widget.apply_render_overrides(overrides);
        }
        widget
    }

    pub fn into_widget(self) -> IslandWidget {
        self.to_widget()
    }
}

impl<Action> Widget for DynamicIsland<Action> {
    fn measure(&self, constraints: Constraints) -> Size {
        self.to_widget().measure(constraints)
    }

    fn paint(&self, rect: Rect, ctx: &mut PaintContext) {
        self.to_widget().paint(rect, ctx);
    }
}

impl<Action> Default for DynamicIsland<Action> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widgets::compact_bar::ChromeVisibility;
    use crate::core::geometry::Rect;

    #[test]
    fn dynamic_island_resolves_target_click_binding() {
        let island = DynamicIsland::new().on_target_click(
            DynamicIslandTarget::Session("session-1".to_string()),
            "open",
        );

        assert_eq!(
            island.action_for_target_gesture(
                &DynamicIslandTarget::Session("session-1".to_string()),
                DynamicIslandGesture::Click
            ),
            Some(&"open")
        );
        assert_eq!(
            island.action_for_target_gesture(
                &DynamicIslandTarget::Session("session-2".to_string()),
                DynamicIslandGesture::Click
            ),
            None
        );
    }

    #[test]
    fn dynamic_island_materializes_spec_with_overrides() {
        let widget = DynamicIsland::<()>::new()
            .mode(DisplayMode::Expanded)
            .layout(IslandWidgetLayout::new(320.0, 52.0, 200.0))
            .chrome(ChromeVisibility::expanded())
            .reveal(0.25, false)
            .child(CompactBar::new().headline("Reef"))
            .child(Card::new(CardStyle::Default).title("Status"))
            .render_overrides(IslandRenderOverrides::new(
                400.0,
                48.0,
                220.0,
                ChromeVisibility::compact(),
                0.75,
                true,
            ))
            .to_widget();

        assert_eq!(widget.mode, DisplayMode::Expanded);
        assert_eq!(widget.width, 400.0);
        assert_eq!(widget.compact_height, 48.0);
        assert_eq!(widget.expanded_height, 220.0);
        assert_eq!(widget.reveal_progress, 0.75);
        assert!(widget.entering);
        assert_eq!(widget.cards.len(), 1);
    }

    #[test]
    fn dynamic_island_widget_measure_and_paint_delegate() {
        let compact = DynamicIsland::<()>::new()
            .mode(DisplayMode::Compact)
            .layout(IslandWidgetLayout::new(320.0, 48.0, 180.0))
            .child(CompactBar::new());
        assert_eq!(
            compact.measure(Constraints::tight(Size {
                width: 320.0,
                height: 48.0,
            })),
            Size {
                width: 320.0,
                height: 48.0,
            }
        );

        let expanded = DynamicIsland::<()>::new()
            .mode(DisplayMode::Expanded)
            .layout(IslandWidgetLayout::new(320.0, 48.0, 180.0))
            .chrome(ChromeVisibility::expanded())
            .child(CompactBar::new())
            .child(Card::new(CardStyle::Default).title("Status").height(80.0));
        let mut primitives = Vec::new();
        let mut ctx = PaintContext {
            primitives: &mut primitives,
        };
        expanded.paint(
            Rect {
                x: 0.0,
                y: 0.0,
                width: 320.0,
                height: 180.0,
            },
            &mut ctx,
        );
        assert!(!primitives.is_empty());

        let hidden = DynamicIsland::<()>::new();
        assert_eq!(
            hidden.measure(Constraints::tight(Size {
                width: 320.0,
                height: 0.0,
            })),
            Size {
                width: 320.0,
                height: 0.0,
            }
        );
    }
}

#[macro_export]
macro_rules! dynamic_island {
    ($($child:expr),* $(,)?) => {{
        let island = $crate::prelude::DynamicIsland::new()
            $(.child($child))*;
        island
    }};
}
