use crate::native_panel_core::{PanelChromeVisibilitySpec, EXPANDED_PANEL_RADIUS};
use crate::native_panel_ui::visual_plan::{
    resolve_compact_chrome_visibility, NativePanelPaintInput, NativePanelVisualDisplayMode,
};
use reef_core::color::Color;
use reef_theme::panel as panel_theme;
use reef_widgets::compact_bar::{ChromeVisibility, CompactBar, CompactShoulder, CompletionGlow};
use reef_widgets::island::ExpandedShell;
use reef_widgets::island_widget::{DisplayMode, IslandWidgetLayout};
use reef_widgets::mascot::MascotWidget;

pub(super) fn display_mode(input: &NativePanelPaintInput) -> DisplayMode {
    if !input.window_state.visible {
        return DisplayMode::Hidden;
    }
    match input.display_mode {
        NativePanelVisualDisplayMode::Hidden => DisplayMode::Hidden,
        NativePanelVisualDisplayMode::Compact => DisplayMode::Compact,
        NativePanelVisualDisplayMode::Expanded => DisplayMode::Expanded,
    }
}

pub(super) fn layout(input: &NativePanelPaintInput) -> IslandWidgetLayout {
    IslandWidgetLayout::new(
        input.panel_frame.width.max(input.compact_bar_frame.width),
        input.compact_bar_frame.height,
        input.panel_frame.height.max(input.shell_frame.height),
    )
}

pub(super) fn chrome_spec(input: &NativePanelPaintInput) -> PanelChromeVisibilitySpec {
    let expanded_display_mode = input.display_mode == NativePanelVisualDisplayMode::Expanded;
    resolve_compact_chrome_visibility(input, expanded_display_mode)
}

pub(super) fn chrome_visibility(
    input: &NativePanelPaintInput,
    spec: PanelChromeVisibilitySpec,
) -> ChromeVisibility {
    ChromeVisibility {
        separator_visibility: input.separator_visibility.clamp(0.0, 1.0),
        shoulder_progress: input.shoulder_progress.clamp(0.0, 1.0),
        collapsed_alpha: 1.0 - spec.collapsed_exit_progress.clamp(0.0, 1.0),
        action_button_visibility: spec.action_buttons.opacity,
    }
}

pub(super) fn compact_bar(
    input: &NativePanelPaintInput,
    layout: IslandWidgetLayout,
    chrome: ChromeVisibility,
    mascot: Option<MascotWidget>,
    glow: Option<CompletionGlow>,
    shoulder_left: Option<CompactShoulder>,
    shoulder_right: Option<CompactShoulder>,
) -> CompactBar {
    let mut compact_bar = CompactBar::new()
        .headline(input.headline_text.clone())
        .headline_emphasized(input.headline_emphasized)
        .counts(input.active_count.clone(), input.total_count.clone())
        .show_actions(input.action_buttons_visible)
        .debug_mode(input.mascot_debug_mode_enabled)
        .chrome(chrome)
        .height(layout.compact_height);
    compact_bar.active_count_scroll = active_count_scroll(input.active_count_elapsed_ms);
    compact_bar.completion_count = input.completion_count;
    compact_bar.mascot = mascot;
    compact_bar.glow = glow;
    compact_bar.shoulder_left = shoulder_left;
    compact_bar.shoulder_right = shoulder_right;
    compact_bar
}

pub(super) fn expanded_shell(input: &NativePanelPaintInput) -> ExpandedShell {
    let mut shell = ExpandedShell::new().radius(EXPANDED_PANEL_RADIUS);
    shell.fill_color = Color::from(panel_theme::SHELL_FILL);
    shell.border_color = Color::from(panel_theme::SHELL_BORDER);
    shell.separator_color = Color::from(panel_theme::SHELL_SEPARATOR);
    if input.separator_visibility > 0.01 {
        shell.separator_y = Some(input.compact_bar_frame.height + 8.0);
    }
    shell
}

fn active_count_scroll(elapsed_ms: u128) -> f64 {
    if elapsed_ms == 0 {
        0.0
    } else {
        ((elapsed_ms as f64) / 180.0).clamp(0.0, 1.0)
    }
}
