use super::dpi::{DpiScale, PhysicalRect};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceKey {
    pub physical_rect: PhysicalRect,
    dpi_scale_millis: i32,
}

impl ResourceKey {
    pub fn new(physical_rect: PhysicalRect, dpi_scale: DpiScale) -> Self {
        Self {
            physical_rect,
            dpi_scale_millis: (dpi_scale.scale * 1000.0).round() as i32,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ResourceCacheState {
    current_key: Option<ResourceKey>,
    rebuild_count: usize,
}

impl ResourceCacheState {
    pub fn sync(&mut self, key: ResourceKey) -> bool {
        if self.current_key == Some(key) {
            return false;
        }
        self.current_key = Some(key);
        self.rebuild_count += 1;
        true
    }

    pub fn rebuild_count(&self) -> usize {
        self.rebuild_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_syncs_on_new_key() {
        let mut cache = ResourceCacheState::default();
        let key = ResourceKey::new(
            PhysicalRect {
                x: 0,
                y: 0,
                width: 100,
                height: 50,
            },
            DpiScale::from_scale(1.0),
        );
        assert!(cache.sync(key));
        assert_eq!(cache.rebuild_count(), 1);
        assert!(!cache.sync(key));
        assert_eq!(cache.rebuild_count(), 1);
    }
}
