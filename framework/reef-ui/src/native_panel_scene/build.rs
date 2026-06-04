use crate::native_panel_core::{
    displayed_default_pending_permissions, displayed_default_pending_questions,
    displayed_prompt_assist_sessions, displayed_sessions, format_status, normalize_status,
    session_title, sync_panel_snapshot_state, ExpandedSurface, PanelHitAction,
    PanelSemanticTarget, PanelSettingsState, PanelState, StatusQueuePayload,
};
use chrono::{DateTime, Utc};
use echoisland_runtime::RuntimeSnapshot;

use super::PanelRuntimeRenderState;

use super::{
    build_compact_bar_scene, build_completion_glow, build_mascot_pose, build_session_surface_scene,
    build_settings_card, build_settings_surface_scene, build_status_surface_scene,
    build_surface_scene, resolve_settings_surface_projection, settings_rows, PanelScene,
    SceneBadge, SceneCard, SceneHitTarget, SceneNode, SceneText,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PanelDisplayOptionState {
    pub index: usize,
    pub key: String,
    pub label: String,
    pub supports_wide_island: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PanelInteractionProfile {
    #[default]
    FullHost,
    Standalone,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PanelSceneBuildInput {
    pub display_options: Vec<PanelDisplayOptionState>,
    pub settings: PanelSettingsState,
    pub app_version: String,
    pub update_status: crate::updater_service::AppUpdateStatus,
    pub interaction_profile: PanelInteractionProfile,
}

#[derive(Clone, Debug)]
pub struct PanelRuntimeSceneBundle {
    pub scene: PanelScene,
    pub runtime_render_state: PanelRuntimeRenderState,
    pub displayed_snapshot: RuntimeSnapshot,
}

impl Default for PanelSceneBuildInput {
    fn default() -> Self {
        Self {
            display_options: vec![fallback_panel_display_option()],
            settings: PanelSettingsState::default(),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            update_status: crate::updater_service::AppUpdateStatus::idle(),
            interaction_profile: PanelInteractionProfile::FullHost,
        }
    }
}

pub fn build_panel_scene(
    state: &PanelState,
    snapshot: &RuntimeSnapshot,
    input: &PanelSceneBuildInput,
) -> PanelScene {
    let compact_bar = build_compact_bar_scene(state, snapshot);
    let surface_scene = build_surface_scene(state, &compact_bar);
    let status_surface = build_status_surface_scene(state, snapshot);
    let session_surface = build_session_surface_scene(state, snapshot);
    let settings_projection =
        resolve_settings_surface_projection(&input.display_options, input.settings);
    let settings_surface = build_settings_surface_scene(
        settings_projection,
        input.settings,
        &input.app_version,
        &input.update_status,
        input.interaction_profile,
    );
    let glow = build_completion_glow(state);
    let mascot_pose = build_mascot_pose(state, snapshot, input.settings.mascot_enabled);
    let mut cards = Vec::new();
    let mut hit_targets = Vec::new();

    match state.surface_mode {
        ExpandedSurface::Settings if state.expanded => {
            let settings = build_settings_card(&settings_surface);
            for row in settings_rows(&settings) {
                hit_targets.push(SceneHitTarget {
                    action: row.action,
                    value: String::new(),
                    semantic_target: None,
                });
            }
            cards.push(settings);
        }
        ExpandedSurface::Status if !state.status_queue.is_empty() => {
            for item in &state.status_queue {
                match &item.payload {
                    StatusQueuePayload::Approval(_) => {
                        cards.push(SceneCard::StatusApproval { item: item.clone() });
                        if input.interaction_profile != PanelInteractionProfile::Standalone {
                            hit_targets.push(SceneHitTarget {
                                action: PanelHitAction::FocusSession,
                                value: item.session_id.clone(),
                                semantic_target: Some(PanelSemanticTarget::Session(
                                    item.session_id.clone(),
                                )),
                            });
                        }
                    }
                    StatusQueuePayload::Question(_) => {
                        cards.push(SceneCard::StatusQuestion { item: item.clone() });
                        if input.interaction_profile != PanelInteractionProfile::Standalone {
                            hit_targets.push(SceneHitTarget {
                                action: PanelHitAction::FocusSession,
                                value: item.session_id.clone(),
                                semantic_target: Some(PanelSemanticTarget::Session(
                                    item.session_id.clone(),
                                )),
                            });
                        }
                    }
                    StatusQueuePayload::Completion(_) => {
                        cards.push(SceneCard::StatusCompletion { item: item.clone() });
                        if input.interaction_profile != PanelInteractionProfile::Standalone {
                            hit_targets.push(SceneHitTarget {
                                action: PanelHitAction::FocusSession,
                                value: item.session_id.clone(),
                                semantic_target: Some(PanelSemanticTarget::Session(
                                    item.session_id.clone(),
                                )),
                            });
                        }
                    }
                }
            }
        }
        _ => {
            let pending_permissions = displayed_default_pending_permissions(snapshot);
            let pending_questions = displayed_default_pending_questions(snapshot);
            let prompt_assist_sessions = displayed_prompt_assist_sessions(snapshot);
            let sessions = displayed_sessions(snapshot, &prompt_assist_sessions);

            for pending in pending_permissions.iter().take(1) {
                cards.push(SceneCard::PendingPermission {
                    pending: pending.clone(),
                    count: snapshot.pending_permission_count.max(1),
                });
                if input.interaction_profile != PanelInteractionProfile::Standalone {
                    hit_targets.push(SceneHitTarget {
                        action: PanelHitAction::FocusSession,
                        value: pending.session_id.clone(),
                        semantic_target: Some(PanelSemanticTarget::Session(
                            pending.session_id.clone(),
                        )),
                    });
                }
            }

            for pending in pending_questions.iter().take(1) {
                cards.push(SceneCard::PendingQuestion {
                    pending: pending.clone(),
                    count: snapshot.pending_question_count.max(1),
                });
                if input.interaction_profile != PanelInteractionProfile::Standalone {
                    hit_targets.push(SceneHitTarget {
                        action: PanelHitAction::FocusSession,
                        value: pending.session_id.clone(),
                        semantic_target: Some(PanelSemanticTarget::Session(
                            pending.session_id.clone(),
                        )),
                    });
                }
            }

            for session in prompt_assist_sessions {
                cards.push(SceneCard::PromptAssist {
                    session: session.clone(),
                });
                if input.interaction_profile != PanelInteractionProfile::Standalone {
                    hit_targets.push(SceneHitTarget {
                        action: PanelHitAction::FocusSession,
                        value: session.session_id.clone(),
                        semantic_target: Some(PanelSemanticTarget::Session(
                            session.session_id.clone(),
                        )),
                    });
                }
            }

            for session in sessions {
                cards.push(SceneCard::Session {
                    session: session.clone(),
                    title: session_title(&session),
                    status: SceneBadge {
                        text: format_status(&session.status),
                        emphasized: normalize_status(&session.status) == "running",
                    },
                    snippet: session
                        .last_assistant_message
                        .clone()
                        .or(session.tool_description.clone()),
                });
                if input.interaction_profile != PanelInteractionProfile::Standalone {
                    hit_targets.push(SceneHitTarget {
                        action: PanelHitAction::FocusSession,
                        value: session.session_id.clone(),
                        semantic_target: Some(PanelSemanticTarget::Session(
                            session.session_id.clone(),
                        )),
                    });
                }
            }

            if cards.is_empty() {
                cards.push(SceneCard::Empty);
            }
        }
    }

    let mut nodes = vec![
        SceneNode::Text(compact_bar.headline.clone()),
        SceneNode::Text(SceneText {
            text: compact_bar.active_count.clone(),
            emphasized: false,
        }),
        SceneNode::Text(SceneText {
            text: compact_bar.total_count.clone(),
            emphasized: false,
        }),
        SceneNode::Mascot(mascot_pose),
    ];
    if let Some(glow) = glow.clone() {
        nodes.push(SceneNode::Glow(glow));
    }
    for card in &cards {
        nodes.push(SceneNode::Card(card.clone()));
    }

    PanelScene {
        surface: state.surface_mode,
        compact_bar,
        surface_scene,
        status_surface,
        session_surface,
        settings_surface,
        cards,
        glow,
        mascot_pose,
        debug_mode_enabled: input.settings.debug_mode_enabled,
        hit_targets,
        nodes,
    }
}

pub fn build_panel_runtime_scene_bundle(
    panel_state: &PanelState,
    displayed_snapshot: &RuntimeSnapshot,
    input: &PanelSceneBuildInput,
) -> PanelRuntimeSceneBundle {
    let scene = build_panel_scene(panel_state, displayed_snapshot, input);
    let runtime_render_state =
        resolve_panel_runtime_render_state(panel_state, Some(displayed_snapshot), input);

    PanelRuntimeSceneBundle {
        scene,
        runtime_render_state,
        displayed_snapshot: displayed_snapshot.clone(),
    }
}

pub fn sync_panel_runtime_scene_bundle(
    panel_state: &mut PanelState,
    raw_snapshot: &RuntimeSnapshot,
    input: &PanelSceneBuildInput,
    now: DateTime<Utc>,
) -> PanelRuntimeSceneBundle {
    let sync_result = sync_panel_snapshot_state(panel_state, raw_snapshot, now);
    build_panel_runtime_scene_bundle(panel_state, &sync_result.displayed_snapshot, input)
}

pub fn resolve_panel_shell_scene_state_for_runtime(
    state: &PanelState,
    snapshot: Option<&RuntimeSnapshot>,
    input: &PanelSceneBuildInput,
) -> super::PanelShellSceneState {
    snapshot
        .map(|snapshot| resolve_panel_shell_scene_state(&build_panel_scene(state, snapshot, input)))
        .unwrap_or_default()
}

pub fn resolve_panel_runtime_render_state(
    state: &PanelState,
    snapshot: Option<&RuntimeSnapshot>,
    input: &PanelSceneBuildInput,
) -> super::PanelRuntimeRenderState {
    super::PanelRuntimeRenderState {
        transitioning: state.transitioning,
        shell_scene: resolve_panel_shell_scene_state_for_runtime(state, snapshot, input),
    }
}

pub fn resolve_panel_shell_scene_state(scene: &PanelScene) -> super::PanelShellSceneState {
    super::PanelShellSceneState {
        surface: scene.surface,
        headline_emphasized: scene.compact_bar.headline.emphasized,
        edge_actions_visible: scene.compact_bar.actions_visible,
    }
}

pub fn panel_display_option_state(
    index: usize,
    key: impl Into<String>,
    name: &str,
    width: u32,
    height: u32,
) -> PanelDisplayOptionState {
    panel_display_option_state_with_width_support(index, key, name, width, height, true)
}

pub fn panel_display_option_state_with_width_support(
    index: usize,
    key: impl Into<String>,
    name: &str,
    width: u32,
    height: u32,
    supports_wide_island: bool,
) -> PanelDisplayOptionState {
    PanelDisplayOptionState {
        index,
        key: key.into(),
        label: panel_display_option_label(name, width, height),
        supports_wide_island,
    }
}

pub fn panel_display_option_label(name: &str, width: u32, height: u32) -> String {
    format!("{name} · {width}×{height}")
}

pub fn fallback_panel_display_option() -> PanelDisplayOptionState {
    PanelDisplayOptionState {
        index: 0,
        key: "default".to_string(),
        label: "Display 1".to_string(),
        supports_wide_island: true,
    }
}
