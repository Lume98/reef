#[cfg(target_os = "windows")]
use std::sync::OnceLock;

#[derive(Clone, Debug, Default)]
pub struct Direct2DFactory {
    #[cfg(target_os = "windows")]
    factory: Option<windows::Win32::Graphics::Direct2D::ID2D1Factory>,
}

impl Direct2DFactory {
    pub fn empty() -> Self {
        Self {
            #[cfg(target_os = "windows")]
            factory: None,
        }
    }

    #[cfg(target_os = "windows")]
    pub fn create() -> Result<Self, String> {
        use windows::Win32::Graphics::Direct2D::{
            D2D1CreateFactory, D2D1_FACTORY_TYPE_SINGLE_THREADED,
        };
        let factory = unsafe {
            D2D1CreateFactory::<windows::Win32::Graphics::Direct2D::ID2D1Factory>(
                D2D1_FACTORY_TYPE_SINGLE_THREADED,
                None,
            )
        }
        .map_err(|e| e.to_string())?;
        Ok(Self {
            factory: Some(factory),
        })
    }

    #[cfg(not(target_os = "windows"))]
    pub fn create() -> Result<Self, String> {
        Ok(Self::empty())
    }

    #[cfg(target_os = "windows")]
    pub fn shared() -> Result<Self, String> {
        static FACTORY: OnceLock<Result<Direct2DFactory, String>> = OnceLock::new();
        FACTORY.get_or_init(Self::create).clone()
    }

    #[cfg(not(target_os = "windows"))]
    pub fn shared() -> Result<Self, String> {
        Self::create()
    }

    pub fn is_initialized(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            self.factory.is_some()
        }
        #[cfg(not(target_os = "windows"))]
        {
            false
        }
    }

    #[cfg(target_os = "windows")]
    pub fn factory(&self) -> Option<&windows::Win32::Graphics::Direct2D::ID2D1Factory> {
        self.factory.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::Direct2DFactory;

    #[test]
    fn factory_can_represent_uninitialized() {
        let factory = Direct2DFactory::empty();
        assert!(!factory.is_initialized());
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn factory_can_initialize_on_windows() {
        let factory = Direct2DFactory::create().expect("create Direct2D factory");
        assert!(factory.is_initialized());
    }
}
