use crate::{Badge, BodyLine, Card, CardStyle};

use super::{short_id::short_id, IslandPendingApprovalInput};

pub(super) fn build_pending_approval_card(pending: &IslandPendingApprovalInput) -> Card {
    Card::new(CardStyle::PendingApproval)
        .title("Approval Required")
        .subtitle(format!("#{} · Approval", short_id(&pending.session_id)))
        .badge(Badge::status("Approval", true))
        .badge(Badge::source(&pending.source))
        .body_line(BodyLine::plain(
            Some("!"),
            pending
                .tool_description
                .clone()
                .unwrap_or_else(|| "Waiting for your approval".to_string()),
        ))
        .action_hint("Allow / Deny in terminal")
        .height(80.0)
}
