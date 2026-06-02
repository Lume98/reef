use reef_core::geometry::{Point, Rect};

/// Type of interactive region.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointerRegionKind {
    Shell,
    CompactBar,
    CardsContainer,
    ActionButton { action: HitAction },
    SettingsRow { index: usize },
    DebugTrigger,
    Mascot,
}

/// Action dispatched on click.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HitAction {
    ToggleSettings,
    Quit,
    FocusSession,
    CycleDisplay,
    CycleWidth,
    CycleLanguage,
    ToggleSound,
    ToggleMascot,
    OpenUpdates,
    DebugMode,
}

/// A clickable region with action dispatch.
#[derive(Clone, Debug)]
pub struct PointerRegion {
    pub kind: PointerRegionKind,
    pub frame: Rect,
    pub action: HitAction,
    pub value: String,
}

impl PointerRegion {
    pub fn new(kind: PointerRegionKind, frame: Rect, action: HitAction, value: impl Into<String>) -> Self {
        Self { kind, frame, action, value: value.into() }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.frame.x
            && point.x <= self.frame.x + self.frame.width
            && point.y >= self.frame.y
            && point.y <= self.frame.y + self.frame.height
    }
}

/// Collection of pointer regions with hit testing.
#[derive(Clone, Debug, Default)]
pub struct PointerRegionSet {
    pub regions: Vec<PointerRegion>,
}

impl PointerRegionSet {
    pub fn new() -> Self {
        Self { regions: Vec::new() }
    }

    pub fn push(&mut self, region: PointerRegion) {
        self.regions.push(region);
    }

    /// Find the top-most region that contains the point (last added wins).
    pub fn hit_test(&self, point: Point) -> Option<&PointerRegion> {
        self.regions.iter().rev().find(|r| r.contains(point))
    }

    pub fn hit_test_kind(&self, point: Point) -> Option<PointerRegionKind> {
        self.hit_test(point).map(|r| r.kind)
    }

    pub fn hit_test_action(&self, point: Point) -> Option<(HitAction, &str)> {
        self.hit_test(point).map(|r| (r.action, r.value.as_str()))
    }

    pub fn clear(&mut self) {
        self.regions.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.regions.is_empty()
    }
}

/// Pointer input state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PointerInput {
    Move { x: f64, y: f64 },
    Click { x: f64, y: f64 },
    Leave,
}

/// Result of a pointer interaction dispatch.
#[derive(Clone, Debug)]
pub enum InteractionResult {
    Hit(HitAction, String),
    HoverInside,
    HoverOutside,
    NoAction,
}

/// Build compact bar pointer regions.
pub fn build_compact_pointer_regions(
    bar_frame: Rect,
    settings_visible: bool,
    quit_visible: bool,
    mascot_frame: Option<Rect>,
) -> PointerRegionSet {
    let mut set = PointerRegionSet::new();

    // Compact bar itself
    set.push(PointerRegion::new(
        PointerRegionKind::CompactBar,
        bar_frame,
        HitAction::FocusSession,
        "",
    ));

    // Settings button
    if settings_visible {
        let settings_frame = Rect { x: bar_frame.x, y: bar_frame.y, width: 44.0, height: bar_frame.height };
        set.push(PointerRegion::new(
            PointerRegionKind::ActionButton { action: HitAction::ToggleSettings },
            settings_frame,
            HitAction::ToggleSettings,
            "",
        ));
        // Debug trigger next to settings
        let debug_frame = Rect { x: bar_frame.x + 44.0, y: bar_frame.y, width: 36.0, height: bar_frame.height };
        set.push(PointerRegion::new(
            PointerRegionKind::DebugTrigger,
            debug_frame,
            HitAction::DebugMode,
            "",
        ));
    }

    // Quit button
    if quit_visible {
        let quit_frame = Rect {
            x: bar_frame.x + bar_frame.width - 44.0,
            y: bar_frame.y,
            width: 44.0,
            height: bar_frame.height,
        };
        set.push(PointerRegion::new(
            PointerRegionKind::ActionButton { action: HitAction::Quit },
            quit_frame,
            HitAction::Quit,
            "",
        ));
    }

    // Mascot
    if let Some(frame) = mascot_frame {
        set.push(PointerRegion::new(PointerRegionKind::Mascot, frame, HitAction::FocusSession, ""));
    }

    set
}

/// Build expanded card pointer regions.
pub fn build_card_pointer_regions(card_frames: &[Rect], card_ids: &[String]) -> PointerRegionSet {
    let mut set = PointerRegionSet::new();
    for (i, frame) in card_frames.iter().enumerate() {
        let value = card_ids.get(i).cloned().unwrap_or_default();
        set.push(PointerRegion::new(
            PointerRegionKind::CardsContainer,
            *frame,
            HitAction::FocusSession,
            value,
        ));
    }
    set
}
