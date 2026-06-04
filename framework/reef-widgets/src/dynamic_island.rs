use crate::{Card, CompactBar, IslandWidget, MascotWidget, ProgressBar};

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
    children: Vec<DynamicIslandChild>,
    actions: DynamicIslandActions<Action>,
}

impl<Action> DynamicIsland<Action> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            actions: DynamicIslandActions::default(),
        }
    }

    pub fn child(mut self, child: impl Into<DynamicIslandChild>) -> Self {
        self.children.push(child.into());
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

    pub fn actions(&self) -> &DynamicIslandActions<Action> {
        &self.actions
    }

    pub fn action_for_gesture(&self, gesture: DynamicIslandGesture) -> Option<&Action> {
        match gesture {
            DynamicIslandGesture::Click => self.actions.click.as_ref(),
            DynamicIslandGesture::Swipe => self.actions.swipe.as_ref(),
            DynamicIslandGesture::Hover => self.actions.hover.as_ref(),
        }
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

    pub fn into_widget(self) -> IslandWidget {
        let mut widget = IslandWidget::new();
        for child in self.children {
            match child {
                DynamicIslandChild::CompactBar(bar) => widget = widget.compact_bar(bar),
                DynamicIslandChild::Card(card) => widget = widget.card(card),
                DynamicIslandChild::Mascot(mascot) => widget = widget.mascot(mascot),
                DynamicIslandChild::ProgressBar(progress) => {
                    widget = widget.card(
                        Card::new(crate::CardStyle::Default)
                            .title("Progress")
                            .height(40.0)
                            .action_hint(format!("{:.0}%", progress.value.clamp(0.0, 1.0) * 100.0)),
                    );
                }
            }
        }
        widget
    }
}

impl<Action> Default for DynamicIsland<Action> {
    fn default() -> Self {
        Self::new()
    }
}

#[macro_export]
macro_rules! dynamic_island {
    ($($child:expr),* $(,)?) => {{
        let island = $crate::DynamicIsland::new()
            $(.child($child))*;
        island
    }};
}
