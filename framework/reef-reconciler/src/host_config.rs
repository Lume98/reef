use reef_vnode::{ElementType, PropsMap};

/// A unique identifier for a host platform instance, produced by the
/// host config during the commit phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HostInstanceId(pub u64);

/// The connector between the reconciler and the platform render layer.
///
/// Analogous to React's `HostConfig` — defines how the reconciler creates,
/// updates, and removes platform-native elements.
pub trait HostConfig {
    /// Create a platform instance for the given element type and props.
    fn create_instance(&mut self, ty: &ElementType, props: &PropsMap) -> HostInstanceId;

    /// Insert a child instance into a parent instance at the given index.
    fn append_child(&mut self, parent: HostInstanceId, child: HostInstanceId);

    /// Update an existing instance with new props.
    fn update_instance(&mut self, instance: HostInstanceId, props: &PropsMap);

    /// Remove an instance from its parent.
    fn remove_instance(&mut self, instance: HostInstanceId);
}
