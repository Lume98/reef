use crate::core::color::Color;
use crate::vnode::VNode;
use std::fmt;

/// Supported property value types for the UI framework.
pub enum PropValue {
    String(String),
    I32(i32),
    F64(f64),
    Bool(bool),
    Color(Color),
    /// An event callback, invoked when the element receives a matching event.
    /// The `&str` argument is the event type (e.g. "click", "pointer_down").
    EventCallback(Box<dyn Fn(&str)>),
    /// A list of child VNodes, used to pass children to function components.
    VNodeList(Vec<VNode>),
}

// Manual implementations for EventCallback variant

impl Clone for PropValue {
    fn clone(&self) -> Self {
        match self {
            PropValue::String(v) => PropValue::String(v.clone()),
            PropValue::I32(v) => PropValue::I32(*v),
            PropValue::F64(v) => PropValue::F64(*v),
            PropValue::Bool(v) => PropValue::Bool(*v),
            PropValue::Color(v) => PropValue::Color(*v),
            PropValue::EventCallback(_) => PropValue::EventCallback(Box::new(|_| {})),
            PropValue::VNodeList(v) => PropValue::VNodeList(v.clone()),
        }
    }
}

impl fmt::Debug for PropValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PropValue::String(v) => write!(f, "String({:?})", v),
            PropValue::I32(v) => write!(f, "I32({})", v),
            PropValue::F64(v) => write!(f, "F64({})", v),
            PropValue::Bool(v) => write!(f, "Bool({})", v),
            PropValue::Color(v) => write!(f, "Color({:?})", v),
            PropValue::EventCallback(_) => write!(f, "EventCallback(<fn>)"),
            PropValue::VNodeList(v) => write!(f, "VNodeList({})", v.len()),
        }
    }
}

impl PartialEq for PropValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PropValue::String(a), PropValue::String(b)) => a == b,
            (PropValue::I32(a), PropValue::I32(b)) => a == b,
            (PropValue::F64(a), PropValue::F64(b)) => a == b,
            (PropValue::Bool(a), PropValue::Bool(b)) => a == b,
            (PropValue::Color(a), PropValue::Color(b)) => a == b,
            (PropValue::EventCallback(_), PropValue::EventCallback(_)) => true,
            (PropValue::VNodeList(a), PropValue::VNodeList(b)) => a == b,
            _ => false,
        }
    }
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
        self.entries.iter().find(|(k, _)| *k == key).map(|(_, v)| v)
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

impl Clone for PropsMap {
    fn clone(&self) -> Self {
        Self {
            entries: self.entries.iter().map(|(k, v)| (*k, v.clone())).collect(),
        }
    }
}

impl fmt::Debug for PropsMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(self.entries.iter().map(|(k, v)| (k, v)))
            .finish()
    }
}

impl PartialEq for PropsMap {
    fn eq(&self, other: &Self) -> bool {
        if self.entries.len() != other.entries.len() {
            return false;
        }
        self.entries
            .iter()
            .all(|(k, v)| other.entries.iter().any(|(ok, ov)| k == ok && v == ov))
    }
}

impl Default for PropsMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn props_map_insert_and_get() {
        let mut map = PropsMap::new();
        map.insert("color", Color::rgb(255, 0, 0));
        map.insert("radius", 12.0_f64);
        map.insert("label", "hello");

        assert_eq!(
            map.get("color"),
            Some(&PropValue::Color(Color::rgb(255, 0, 0)))
        );
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
}
