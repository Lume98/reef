use crate::{BodyLine, Card, CardStyle};

pub(super) fn build_empty_sessions_card() -> Card {
    Card::new(CardStyle::Empty)
        .title("No active sessions")
        .body_line(BodyLine::plain(
            None,
            "Reef UI is watching for new activity.",
        ))
        .height(60.0)
}
