use reef_core::color::Color;

/// Supported property value types for the UI framework.
#[derive(Clone, Debug, PartialEq)]
pub enum PropValue {
    String(String),
    I32(i32),
    F64(f64),
    Bool(bool),
    Color(Color),
}

impl From<String> for PropValue {
    fn from(v: String) -> Self {
        PropValue::String(v)
    }
}

impl From<&str> for PropValue {
    fn from(v: &str) -> Self {
        PropValue::String(v.to_string())
    }
}

impl From<f64> for PropValue {
    fn from(v: f64) -> Self {
        PropValue::F64(v)
    }
}

impl From<i32> for PropValue {
    fn from(v: i32) -> Self {
        PropValue::I32(v)
    }
}

impl From<bool> for PropValue {
    fn from(v: bool) -> Self {
        PropValue::Bool(v)
    }
}

impl From<Color> for PropValue {
    fn from(v: Color) -> Self {
        PropValue::Color(v)
    }
}

/// A flat key-value property map used by VElement.
///
/// Props are stored as a `Vec` of `(key, value)` pairs for fast construction
/// in proc-macros and efficient iteration during reconciliation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PropsMap {
    entries: Vec<(&'static str, PropValue)>,
}

impl PropsMap {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Create from a pre-built vec of pairs — used by the `rsx!` proc-macro.
    pub fn from_pairs(pairs: Vec<(&'static str, PropValue)>) -> Self {
        Self { entries: pairs }
    }

    pub fn insert(&mut self, key: &'static str, value: impl Into<PropValue>) {
        self.entries.push((key, value.into()));
    }

    pub fn get(&self, key: &str) -> Option<&PropValue> {
        self.entries
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v)
    }

    pub fn iter(&self) -> impl Iterator<Item = &(&'static str, PropValue)> {
        self.entries.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reef_core::color::Color;

    #[test]
    fn props_map_insert_and_get() {
        let mut map = PropsMap::new();
        map.insert("color", Color::rgb(255, 0, 0));
        map.insert("radius", 12.0_f64);
        map.insert("label", "hello");

        assert_eq!(map.get("color"), Some(&PropValue::Color(Color::rgb(255, 0, 0))));
        assert_eq!(map.get("radius"), Some(&PropValue::F64(12.0)));
        assert_eq!(map.get("label"), Some(&PropValue::String("hello".into())));
        assert_eq!(map.get("missing"), None);
    }

    #[test]
    fn props_map_from_pairs() {
        let map = PropsMap::from_pairs(vec![
            ("color", PropValue::Color(Color::rgb(18, 18, 22))),
            ("radius", PropValue::F64(24.0)),
        ]);
        assert_eq!(map.len(), 2);
        assert!(map.get("color").is_some());
        assert!(map.get("radius").is_some());
    }

    #[test]
    fn prop_value_from_conversions() {
        assert_eq!(PropValue::from("text"), PropValue::String("text".into()));
        assert_eq!(PropValue::from(42_i32), PropValue::I32(42));
        assert_eq!(PropValue::from(3.14_f64), PropValue::F64(3.14));
        assert_eq!(PropValue::from(true), PropValue::Bool(true));
        assert_eq!(
            PropValue::from(Color::rgb(10, 20, 30)),
            PropValue::Color(Color::rgb(10, 20, 30))
        );
    }
}
