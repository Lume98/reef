pub(crate) use reef_ui::native_panel_ui::visual::{
    native_panel_visual_card_input_from_scene_card,
    native_panel_visual_card_input_from_scene_card_with_height,
    NativePanelVisualActionButtonInput, NativePanelVisualCardBadgeInput,
    NativePanelVisualCardBodyLineInput, NativePanelVisualCardBodyRole,
    NativePanelVisualCardInput, NativePanelVisualCardRowInput, NativePanelVisualCardStyle,
    NativePanelVisualDisplayMode, NativePanelVisualPlan, NativePanelVisualPlanInput,
};

pub(crate) fn resolve_native_panel_visual_plan(
    input: &NativePanelVisualPlanInput,
) -> NativePanelVisualPlan {
    disable_mascot_sprite_in_tests();
    reef_ui::native_panel_ui::visual::resolve_native_panel_visual_plan(input)
}

pub(crate) fn resolve_native_panel_compact_bar_visual_plan(
    input: &NativePanelVisualPlanInput,
) -> NativePanelVisualPlan {
    disable_mascot_sprite_in_tests();
    reef_ui::native_panel_ui::visual::resolve_native_panel_compact_bar_visual_plan(input)
}

#[cfg(test)]
fn disable_mascot_sprite_in_tests() {
    std::env::set_var("ECHOISLAND_MASCOT_SPRITE", "0");
}

#[cfg(not(test))]
fn disable_mascot_sprite_in_tests() {}

