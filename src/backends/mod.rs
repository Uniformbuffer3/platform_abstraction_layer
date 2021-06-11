#[cfg(target_os = "linux")]
mod linux;

pub struct Platform {
    #[cfg(target_os = "linux")]
    backend: linux::LinuxPlatform,
}
impl Platform {
    pub fn new(contexts: Vec<crate::definitions::ExternalContext>) -> Self {
        #[cfg(target_os = "linux")]
        let backend = linux::LinuxPlatform::new(contexts).unwrap();

        Self { backend }
    }
}

#[cfg(target_os = "linux")]
impl std::ops::Deref for Platform {
    type Target = linux::LinuxPlatform;

    fn deref(&self) -> &Self::Target {
        &self.backend
    }
}
#[cfg(target_os = "linux")]
impl std::ops::DerefMut for Platform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.backend
    }
}
