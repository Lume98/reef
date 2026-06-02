use crate::native_panel_core::{
    absolute_rect, compose_local_rect, point_in_rect, resolve_compact_action_button_layout,
    resolve_settings_surface_card_height, settings_surface_row_frame, PanelHitTarget, PanelLayout,
    PanelPoint, PanelRect,
};
use crate::native_panel_scene::{PanelScene, SceneCard, SceneHitTarget};

use super::commands::{NativePanelPlatformEvent, NativePanelPointerInput};

// ---- pointer region types ----

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NativePanelPointerRegionKind {
    Shell,
    CompactBar,
    CardsContainer,
    DebugModeTrigger,
    EdgeAction(NativePanelEdgeAction),
    HitTarget(PanelHitTarget),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativePanelEdgeAction {
    Settings,
    Quit,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NativePanelPointerRegion {
    pub frame: PanelRect,
    pub kind: NativePanelPointerRegionKind,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NativePanelInteractionPlan {
    pub pointer_regions: Vec<NativePanelPointerRegion>,
}

impl NativePanelInteractionPlan {
    pub fn from_pointer_regions(regions: &[NativePanelPointerRegion]) -> Self {
        Self {
            pointer_regions: regions.to_vec(),
        }
    }

    pub fn pointer_region_at_point(&self, point: PanelPoint) -> Option<&NativePanelPointerRegion> {
        native_panel_pointer_region_at_point(&self.pointer_regions, point)
    }

    pub fn inside_regions(&self, point: PanelPoint) -> bool {
        self.pointer_region_at_point(point).is_some()
    }

    pub fn pointer_state_at_point(&self, point: PanelPoint) -> NativePanelPointerPointState {
        native_panel_pointer_state_at_point(&self.pointer_regions, point)
    }

    pub fn platform_event_at_point(&self, point: PanelPoint) -> Option<NativePanelPlatformEvent> {
        native_panel_platform_event_at_point(&self.pointer_regions, point)
    }

    pub fn input_outcome(&self, input: NativePanelPointerInput) -> NativePanelPointerInputOutcome {
        native_panel_pointer_input_outcome(&self.pointer_regions, input)
    }

    pub fn inside_for_input(&self, input: NativePanelPointerInput) -> Option<bool> {
        native_panel_pointer_inside_for_input(&self.pointer_regions, input)
    }

    pub fn hit_target_at_point(&self, point: PanelPoint) -> Option<PanelHitTarget> {
        native_panel_hit_target_at_point(&self.pointer_regions, point)
    }

    pub fn queue_platform_event_at_point(
        &self,
        events: &mut Vec<NativePanelPlatformEvent>,
        point: PanelPoint,
    ) -> Option<NativePanelPlatformEvent> {
        queue_native_panel_platform_event(events, self.platform_event_at_point(point))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativePanelPointerPointState {
    pub inside: bool,
    pub platform_event: Option<NativePanelPlatformEvent>,
    pub hit_target: Option<PanelHitTarget>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NativePanelEdgeActionFrames {
    pub settings_action: Option<PanelRect>,
    pub quit_action: Option<PanelRect>,
}

impl NativePanelEdgeActionFrames {
    fn edge_action_frame(self, action: NativePanelEdgeAction) -> Option<PanelRect> {
        match action {
            NativePanelEdgeAction::Settings => self.settings_action,
            NativePanelEdgeAction::Quit => self.quit_action,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NativePanelPointerRegionInput {
    pub edge_action_frames: NativePanelEdgeActionFrames,
}

// ---- pointer input outcome ----

use super::commands::NativePanelPointerInputOutcome;

// ---- pointer region resolution ----

pub fn native_panel_platform_event_for_pointer_region(
    region: &NativePanelPointerRegion,
) -> Option<NativePanelPlatformEvent> {
    match &region.kind {
        NativePanelPointerRegionKind::EdgeAction(NativePanelEdgeAction::Settings) => {
            Some(NativePanelPlatformEvent::ToggleSettingsSurface)
        }
        NativePanelPointerRegionKind::EdgeAction(NativePanelEdgeAction::Quit) => {
            Some(NativePanelPlatformEvent::QuitApplication)
        }
        NativePanelPointerRegionKind::DebugModeTrigger => {
            Some(NativePanelPlatformEvent::DebugModeTrigger)
        }
        NativePanelPointerRegionKind::HitTarget(target) => Some(
            super::commands::native_panel_platform_event_for_hit_target(target),
        ),
        NativePanelPointerRegionKind::Shell
        | NativePanelPointerRegionKind::CompactBar
        | NativePanelPointerRegionKind::CardsContainer => None,
    }
}

pub fn native_panel_pointer_region_at_point(
    regions: &[NativePanelPointerRegion],
    point: PanelPoint,
) -> Option<&NativePanelPointerRegion> {
    regions
        .iter()
        .rev()
        .find(|region| point_in_rect(point, region.frame))
}

pub fn native_panel_pointer_inside_regions(
    regions: &[NativePanelPointerRegion],
    point: PanelPoint,
) -> bool {
    native_panel_pointer_region_at_point(regions, point).is_some()
}

pub fn native_panel_platform_event_at_point(
    regions: &[NativePanelPointerRegion],
    point: PanelPoint,
) -> Option<NativePanelPlatformEvent> {
    native_panel_pointer_region_at_point(regions, point)
        .and_then(native_panel_platform_event_for_pointer_region)
}

pub fn queue_native_panel_platform_event(
    events: &mut Vec<NativePanelPlatformEvent>,
    event: Option<NativePanelPlatformEvent>,
) -> Option<NativePanelPlatformEvent> {
    if let Some(event) = event.clone() {
        events.push(event);
    }
    event
}

pub fn queue_native_panel_platform_event_for_pointer_region(
    events: &mut Vec<NativePanelPlatformEvent>,
    region: &NativePanelPointerRegion,
) -> Option<NativePanelPlatformEvent> {
    queue_native_panel_platform_event(
        events,
        native_panel_platform_event_for_pointer_region(region),
    )
}

pub fn queue_native_panel_platform_event_at_point(
    events: &mut Vec<NativePanelPlatformEvent>,
    regions: &[NativePanelPointerRegion],
    point: PanelPoint,
) -> Option<NativePanelPlatformEvent> {
    queue_native_panel_platform_event(events, native_panel_platform_event_at_point(regions, point))
}

pub fn native_panel_pointer_state_at_point(
    regions: &[NativePanelPointerRegion],
    point: PanelPoint,
) -> NativePanelPointerPointState {
    let region = native_panel_pointer_region_at_point(regions, point);
    NativePanelPointerPointState {
        inside: region.is_some(),
        platform_event: region.and_then(native_panel_platform_event_for_pointer_region),
        hit_target: match region.map(|region| &region.kind) {
            Some(NativePanelPointerRegionKind::HitTarget(target)) => Some(target.clone()),
            _ => None,
        },
    }
}

pub fn native_panel_pointer_inside_for_input(
    regions: &[NativePanelPointerRegion],
    input: NativePanelPointerInput,
) -> Option<bool> {
    match input {
        NativePanelPointerInput::Move(point) => {
            Some(native_panel_pointer_inside_regions(regions, point))
        }
        NativePanelPointerInput::Leave => Some(false),
        NativePanelPointerInput::Click(_) => None,
    }
}

pub fn native_panel_platform_event_for_pointer_input(
    regions: &[NativePanelPointerRegion],
    input: NativePanelPointerInput,
) -> Option<NativePanelPlatformEvent> {
    match input {
        NativePanelPointerInput::Click(point) => {
            native_panel_platform_event_at_point(regions, point)
        }
        NativePanelPointerInput::Move(_) | NativePanelPointerInput::Leave => None,
    }
}

pub fn native_panel_hit_target_at_point(
    regions: &[NativePanelPointerRegion],
    point: PanelPoint,
) -> Option<PanelHitTarget> {
    match &native_panel_pointer_region_at_point(regions, point)?.kind {
        NativePanelPointerRegionKind::HitTarget(target) => Some(target.clone()),
        NativePanelPointerRegionKind::Shell
        | NativePanelPointerRegionKind::CompactBar
        | NativePanelPointerRegionKind::CardsContainer
        | NativePanelPointerRegionKind::DebugModeTrigger
        | NativePanelPointerRegionKind::EdgeAction(_) => None,
    }
}

use crate::native_panel_core::HoverTransition;

pub fn native_panel_pointer_input_outcome(
    regions: &[NativePanelPointerRegion],
    input: NativePanelPointerInput,
) -> NativePanelPointerInputOutcome {
    match input {
        NativePanelPointerInput::Move(point) => NativePanelPointerInputOutcome::Hover(
            native_panel_pointer_inside_regions(regions, point).then_some(HoverTransition::Expand),
        ),
        NativePanelPointerInput::Leave => {
            NativePanelPointerInputOutcome::Hover(Some(HoverTransition::Collapse))
        }
        NativePanelPointerInput::Click(point) => NativePanelPointerInputOutcome::Click(
            native_panel_platform_event_at_point(regions, point),
        ),
    }
}

// ---- interaction plan builder ----

pub fn resolve_native_panel_pointer_regions(
    layout: PanelLayout,
    scene: &PanelScene,
    input: Option<NativePanelPointerRegionInput>,
) -> Vec<NativePanelPointerRegion> {
    resolve_native_panel_interaction_plan(layout, scene, input).pointer_regions
}

pub fn resolve_native_panel_interaction_plan(
    layout: PanelLayout,
    scene: &PanelScene,
    input: Option<NativePanelPointerRegionInput>,
) -> NativePanelInteractionPlan {
    let mut regions = Vec::new();

    push_region(
        &mut regions,
        absolute_panel_rect(layout, layout.pill_frame),
        NativePanelPointerRegionKind::CompactBar,
    );
    push_mascot_bubble_hover_region(&mut regions, layout, scene);

    if layout.shell_visible {
        push_region(
            &mut regions,
            absolute_panel_rect(layout, layout.expanded_frame),
            NativePanelPointerRegionKind::Shell,
        );
        push_expanded_debug_mode_trigger_region(
            &mut regions,
            layout,
            input.unwrap_or_default().edge_action_frames,
        );
        push_expanded_top_gap_region(&mut regions, layout);
        push_region(
            &mut regions,
            absolute_expanded_rect(layout, layout.cards_frame),
            NativePanelPointerRegionKind::CardsContainer,
        );
        if scene.compact_bar.actions_visible {
            push_edge_action_regions(
                &mut regions,
                layout,
                input.unwrap_or_default().edge_action_frames,
            );
        }
        push_scene_hit_target_regions(&mut regions, layout, scene);
    }

    NativePanelInteractionPlan {
        pointer_regions: regions,
    }
}

fn push_expanded_debug_mode_trigger_region(
    regions: &mut Vec<NativePanelPointerRegion>,
    layout: PanelLayout,
    edge_action_frames: NativePanelEdgeActionFrames,
) {
    let pill = absolute_panel_rect(layout, layout.pill_frame);
    let action_layout = edge_action_frames
        .edge_action_frame(NativePanelEdgeAction::Settings)
        .unwrap_or_else(|| resolve_compact_action_button_layout(pill).settings);
    let trigger_size = 36.0;
    let trigger_gap = 6.0;
    push_region(
        regions,
        PanelRect {
            x: action_layout.x + action_layout.width + trigger_gap,
            y: pill.y + (pill.height - trigger_size) / 2.0,
            width: trigger_size,
            height: trigger_size,
        },
        NativePanelPointerRegionKind::DebugModeTrigger,
    );
}

fn push_expanded_top_gap_region(regions: &mut Vec<NativePanelPointerRegion>, layout: PanelLayout) {
    let gap_y = layout.expanded_frame.y + layout.expanded_frame.height;
    let gap_height = (layout.content_frame.height - gap_y).max(0.0);
    if gap_height <= 0.0 {
        return;
    }
    push_region(
        regions,
        absolute_panel_rect(
            layout,
            PanelRect {
                x: layout.expanded_frame.x,
                y: gap_y,
                width: layout.expanded_frame.width,
                height: gap_height,
            },
        ),
        NativePanelPointerRegionKind::Shell,
    );
}

fn push_mascot_bubble_hover_region(
    regions: &mut Vec<NativePanelPointerRegion>,
    layout: PanelLayout,
    scene: &PanelScene,
) {
    let has_bubble = scene.compact_bar.completion_count > 0
        || scene.mascot_pose == crate::native_panel_scene::SceneMascotPose::MessageBubble;
    if !has_bubble {
        return;
    }

    let pill = absolute_panel_rect(layout, layout.pill_frame);
    push_region(
        regions,
        PanelRect {
            x: pill.x + 20.0,
            y: pill.y + pill.height - 3.0,
            width: 30.0,
            height: 18.0,
        },
        NativePanelPointerRegionKind::CompactBar,
    );
}

fn push_edge_action_regions(
    regions: &mut Vec<NativePanelPointerRegion>,
    layout: PanelLayout,
    edge_action_frames: NativePanelEdgeActionFrames,
) {
    let pill = absolute_panel_rect(layout, layout.pill_frame);
    let action_layout = resolve_compact_action_button_layout(pill);
    let settings_frame = edge_action_frames
        .edge_action_frame(NativePanelEdgeAction::Settings)
        .unwrap_or_else(|| edge_action_hit_frame(action_layout.settings, pill));
    let quit_frame = edge_action_frames
        .edge_action_frame(NativePanelEdgeAction::Quit)
        .unwrap_or_else(|| edge_action_hit_frame(action_layout.quit, pill));
    push_region(
        regions,
        settings_frame,
        NativePanelPointerRegionKind::EdgeAction(NativePanelEdgeAction::Settings),
    );
    push_region(
        regions,
        quit_frame,
        NativePanelPointerRegionKind::EdgeAction(NativePanelEdgeAction::Quit),
    );
}

fn edge_action_hit_frame(icon_frame: PanelRect, pill: PanelRect) -> PanelRect {
    let horizontal_padding = 5.0;
    PanelRect {
        x: icon_frame.x - horizontal_padding,
        y: pill.y,
        width: icon_frame.width + horizontal_padding * 2.0,
        height: pill.height,
    }
}

fn push_scene_hit_target_regions(
    regions: &mut Vec<NativePanelPointerRegion>,
    layout: PanelLayout,
    scene: &PanelScene,
) {
    if scene.hit_targets.is_empty() {
        return;
    }

    let cards = absolute_expanded_rect(layout, layout.cards_frame);
    if push_settings_hit_target_regions(regions, cards, scene) {
        return;
    }

    let target_count = scene.hit_targets.len();
    let row_height = cards.height / target_count as f64;
    for (index, target) in scene.hit_targets.iter().cloned().enumerate() {
        push_region(
            regions,
            PanelRect {
                x: cards.x,
                y: cards.y + cards.height - row_height * (index + 1) as f64,
                width: cards.width,
                height: row_height,
            },
            NativePanelPointerRegionKind::HitTarget(target.into()),
        );
    }
}

fn push_settings_hit_target_regions(
    regions: &mut Vec<NativePanelPointerRegion>,
    cards: PanelRect,
    scene: &PanelScene,
) -> bool {
    let Some(SceneCard::Settings { rows, .. }) = scene.cards.first() else {
        return false;
    };
    let card_height = resolve_settings_surface_card_height(rows.len());
    let card_frame = PanelRect {
        x: cards.x,
        y: cards.y - (card_height - cards.height).max(0.0),
        width: cards.width,
        height: card_height,
    };
    for (index, target) in scene.hit_targets.iter().cloned().enumerate() {
        push_settings_row_hit_target_regions(regions, card_frame, index, rows.get(index), target);
    }
    true
}

fn push_settings_row_hit_target_regions(
    regions: &mut Vec<NativePanelPointerRegion>,
    card_frame: PanelRect,
    index: usize,
    row: Option<&crate::native_panel_scene::SettingsRowScene>,
    target: SceneHitTarget,
) {
    let row_frame = settings_surface_row_frame(card_frame, index);
    push_region(
        regions,
        row_frame,
        NativePanelPointerRegionKind::HitTarget(target.clone().into()),
    );
    if let Some(row) = row {
        use crate::native_panel_ui::card_visual_spec::{
            card_visual_settings_row_layout, CardVisualRowSpec,
        };
        if let Some(layout) = card_visual_settings_row_layout(
            card_frame,
            index,
            &CardVisualRowSpec {
                title: row.title.clone(),
                value: row.value.text.clone(),
                active: row.value.emphasized,
            },
        ) {
            push_region(
                regions,
                layout.value_badge_frame,
                NativePanelPointerRegionKind::HitTarget(target.into()),
            );
        }
    }
}

pub(super) fn absolute_panel_rect(layout: PanelLayout, local_frame: PanelRect) -> PanelRect {
    absolute_rect(layout.panel_frame, local_frame)
}

pub(super) fn absolute_expanded_rect(layout: PanelLayout, local_frame: PanelRect) -> PanelRect {
    absolute_panel_rect(
        layout,
        compose_local_rect(layout.expanded_frame, local_frame),
    )
}

fn push_region(
    regions: &mut Vec<NativePanelPointerRegion>,
    frame: PanelRect,
    kind: NativePanelPointerRegionKind,
) {
    if frame.width <= 0.0 || frame.height <= 0.0 {
        return;
    }
    regions.push(NativePanelPointerRegion { frame, kind });
}
