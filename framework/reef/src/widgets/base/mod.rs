mod animation;
mod container;
mod interaction;
mod label;

pub use animation::{
    card_content_visibility, collapsed_alpha, ease_in_cubic, ease_out_cubic, lerp,
    shell_reveal_frame, shoulder_progress_from_width, staggered_card_phase, AnimatedValue,
    AnimationTarget, PANEL_CARD_CONTENT_EARLY_EXIT_PROGRESS,
    PANEL_CARD_CONTENT_REVEAL_DELAY_PROGRESS, PANEL_CARD_EXIT_MS, PANEL_CARD_EXIT_STAGGER_MS,
    PANEL_CARD_REVEAL_MS, PANEL_CARD_REVEAL_STAGGER_MS,
};
pub use container::Container;
pub use interaction::{
    build_card_pointer_regions, build_compact_pointer_regions, HitAction, InteractionResult,
    PointerInput, PointerRegion, PointerRegionKind, PointerRegionSet,
};
pub use label::Label;
