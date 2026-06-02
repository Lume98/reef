use echoisland_runtime::{PendingPermissionView, PendingQuestionView, SessionSnapshotView};

use crate::{
    native_panel_core::{
        completion_preview_text, default_panel_card_metric_constants, display_snippet,
        is_long_idle_session, resolve_card_chat_body_width, resolve_completion_card_height,
        resolve_pending_like_card_height, resolve_session_card_content_height,
        resolve_session_card_height, resolve_settings_surface_card_height,
        resolve_stacked_cards_total_height, session_has_visible_card_body, session_prompt_preview,
        session_reply_preview, session_tool_preview, SessionCardContentInput, StatusQueueItem,
        StatusQueuePayload, DEFAULT_PANEL_CANVAS_WIDTH, EMPTY_CARD_HEIGHT, EXPANDED_CARD_GAP,
        PENDING_QUESTION_CARD_MAX_HEIGHT, PENDING_QUESTION_CARD_MIN_HEIGHT,
    },
    native_panel_scene::{resolve_scene_card_height_input, PanelScene, SceneCard},
};

use super::NativePanelPresentationMetrics;

pub fn resolve_native_panel_presentation_metrics(
    scene: &PanelScene,
) -> NativePanelPresentationMetrics {
    use crate::native_panel_core::EXPANDED_MAX_BODY_HEIGHT;
    let expanded_content_height = estimated_scene_content_height(scene);
    NativePanelPresentationMetrics {
        expanded_content_height,
        expanded_body_height: expanded_content_height.min(EXPANDED_MAX_BODY_HEIGHT),
    }
}

pub fn estimated_scene_content_height(scene: &PanelScene) -> f64 {
    estimated_scene_content_height_for_card_width(
        scene,
        crate::native_panel_core::resolve_expanded_cards_width(
            DEFAULT_PANEL_CANVAS_WIDTH,
            crate::native_panel_core::EXPANDED_CARDS_SIDE_INSET,
        ),
    )
}

pub fn estimated_scene_content_height_for_card_width(scene: &PanelScene, card_width: f64) -> f64 {
    estimated_scene_cards_content_height_for_card_width(&scene.cards, card_width)
}

pub fn estimated_scene_cards_content_height(cards: &[SceneCard]) -> f64 {
    estimated_scene_cards_content_height_for_card_width(
        cards,
        crate::native_panel_core::resolve_expanded_cards_width(
            DEFAULT_PANEL_CANVAS_WIDTH,
            crate::native_panel_core::EXPANDED_CARDS_SIDE_INSET,
        ),
    )
}

pub fn estimated_scene_cards_content_height_for_card_width(
    cards: &[SceneCard],
    card_width: f64,
) -> f64 {
    let card_heights = cards
        .iter()
        .map(|card| estimated_scene_card_height_for_card_width(card, card_width))
        .collect::<Vec<_>>();
    resolve_stacked_cards_total_height(&card_heights, EXPANDED_CARD_GAP, EMPTY_CARD_HEIGHT)
}

pub fn estimated_scene_card_height(card: &SceneCard) -> f64 {
    estimated_scene_card_height_for_card_width(
        card,
        crate::native_panel_core::resolve_expanded_cards_width(
            DEFAULT_PANEL_CANVAS_WIDTH,
            crate::native_panel_core::EXPANDED_CARDS_SIDE_INSET,
        ),
    )
}

pub fn estimated_scene_card_height_for_card_width(card: &SceneCard, card_width: f64) -> f64 {
    match resolve_scene_card_height_input(card) {
        crate::native_panel_scene::SceneCardHeightInput::Settings { row_count } => {
            resolve_settings_surface_card_height(row_count)
        }
        crate::native_panel_scene::SceneCardHeightInput::PendingPermission(pending) => {
            pending_permission_card_height(pending, card_width)
        }
        crate::native_panel_scene::SceneCardHeightInput::PendingQuestion(pending) => {
            pending_question_card_height(pending, card_width)
        }
        crate::native_panel_scene::SceneCardHeightInput::PromptAssist(session) => {
            prompt_assist_card_height(session, card_width)
        }
        crate::native_panel_scene::SceneCardHeightInput::Session(session) => {
            session_card_height(session, card_width)
        }
        crate::native_panel_scene::SceneCardHeightInput::StatusItem(item) => {
            status_queue_card_height(item, card_width)
        }
        crate::native_panel_scene::SceneCardHeightInput::Empty => EMPTY_CARD_HEIGHT,
    }
}

fn status_queue_card_height(item: &StatusQueueItem, card_width: f64) -> f64 {
    match &item.payload {
        StatusQueuePayload::Approval(pending) => {
            pending_permission_card_height(pending, card_width)
        }
        StatusQueuePayload::Question(pending) => pending_question_card_height(pending, card_width),
        StatusQueuePayload::Completion(session) => completion_card_height(session, card_width),
    }
}

fn pending_permission_card_height(pending: &PendingPermissionView, card_width: f64) -> f64 {
    let body = display_snippet(pending.tool_description.as_deref(), 78)
        .unwrap_or_else(|| "Waiting for your approval".to_string());
    pending_like_card_height(
        &body,
        crate::native_panel_core::PENDING_PERMISSION_CARD_MIN_HEIGHT,
        crate::native_panel_core::PENDING_PERMISSION_CARD_MAX_HEIGHT,
        card_width,
    )
}

fn pending_question_card_height(pending: &PendingQuestionView, card_width: f64) -> f64 {
    let body = display_snippet(Some(&pending.text), 82)
        .unwrap_or_else(|| "Waiting for your answer".to_string());
    let min_height = if pending.options.is_empty() {
        PENDING_QUESTION_CARD_MIN_HEIGHT
    } else {
        PENDING_QUESTION_CARD_MIN_HEIGHT
            + crate::native_panel_core::PENDING_QUESTION_CARD_OPTIONS_EXTRA_HEIGHT
    };
    pending_like_card_height(
        &body,
        min_height,
        crate::native_panel_core::PENDING_QUESTION_CARD_FALLBACK_MAX_HEIGHT
            .max(PENDING_QUESTION_CARD_MAX_HEIGHT),
        card_width,
    )
}

fn prompt_assist_card_height(_session: &SessionSnapshotView, card_width: f64) -> f64 {
    pending_like_card_height(
        "A command may be waiting for approval in the Codex terminal. Allow or deny it there.",
        crate::native_panel_core::PROMPT_ASSIST_CARD_MIN_HEIGHT,
        crate::native_panel_core::PROMPT_ASSIST_CARD_MAX_HEIGHT,
        card_width,
    )
}

fn completion_card_height(session: &SessionSnapshotView, card_width: f64) -> f64 {
    resolve_completion_card_height(
        &completion_preview_text(session),
        chat_body_width_for_card_width(card_width),
        default_panel_card_metric_constants(),
    )
}

fn pending_like_card_height(body: &str, min_height: f64, max_height: f64, card_width: f64) -> f64 {
    resolve_pending_like_card_height(
        body,
        min_height,
        max_height,
        chat_body_width_for_card_width(card_width),
        default_panel_card_metric_constants(),
    )
}

fn session_card_height(session: &SessionSnapshotView, card_width: f64) -> f64 {
    if is_long_idle_session(session) || !session_has_visible_card_body(session) {
        return 58.0;
    }

    let prompt = session_prompt_preview(session);
    let reply = session_reply_preview(session);
    let body_width = chat_body_width_for_card_width(card_width);
    let content_height = resolve_session_card_content_height(SessionCardContentInput {
        prompt: prompt.as_deref(),
        reply: reply.as_deref(),
        has_tool: session_tool_preview(session).is_some(),
        default_body_width: body_width,
        metrics: default_panel_card_metric_constants(),
    });
    resolve_session_card_height(
        false,
        true,
        content_height,
        default_panel_card_metric_constants(),
    )
}

fn chat_body_width_for_card_width(card_width: f64) -> f64 {
    resolve_card_chat_body_width(card_width, default_panel_card_metric_constants())
}
