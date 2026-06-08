pub(super) fn short_id(id: &str) -> String {
    id.chars().take(6).collect()
}
