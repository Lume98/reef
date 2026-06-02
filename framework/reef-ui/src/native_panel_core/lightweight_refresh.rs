#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativePanelLightweightRefreshInput {
    pub transitioning: bool,
    pub animation_active: bool,
    pub active_count_marquee_needs_refresh: bool,
    pub mascot_animation_needs_refresh: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativePanelLightweightRefreshPlan {
    pub active_count_marquee: NativePanelLightweightRefreshChannelPlan,
    pub mascot_animation: NativePanelLightweightRefreshChannelPlan,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativePanelLightweightRefreshChannelPlan {
    pub refresh_allowed: bool,
    pub reset_timer: bool,
}

pub fn resolve_native_panel_lightweight_refresh_plan(
    input: NativePanelLightweightRefreshInput,
) -> NativePanelLightweightRefreshPlan {
    let suspended = input.transitioning || input.animation_active;
    NativePanelLightweightRefreshPlan {
        active_count_marquee: resolve_native_panel_lightweight_refresh_channel(
            suspended,
            input.active_count_marquee_needs_refresh,
        ),
        mascot_animation: resolve_native_panel_lightweight_refresh_channel(
            suspended,
            input.mascot_animation_needs_refresh,
        ),
    }
}

fn resolve_native_panel_lightweight_refresh_channel(
    suspended: bool,
    needs_refresh: bool,
) -> NativePanelLightweightRefreshChannelPlan {
    NativePanelLightweightRefreshChannelPlan {
        refresh_allowed: !suspended && needs_refresh,
        reset_timer: suspended || !needs_refresh,
    }
}
