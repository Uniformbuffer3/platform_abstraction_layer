pub mod partial_backends;

#[cfg(all(target_os = "linux",feature="linux_platform"))]
mod linux;

#[cfg(feature="state_tracker")]
mod state_tracker;
#[cfg(feature="state_tracker")]
use state_tracker::StateTracker;

#[cfg(feature="post_processing")]
mod post_processing;
#[cfg(feature="post_processing")]
use post_processing::PostProcessing;

use crate::definitions::*;

/**
Entry point of the library, it allows the user to manage input events,
monitor outputs and surface creations under a unified abstraction.
*/
pub struct Platform {
    #[cfg(all(target_os = "linux",feature="linux_platform"))]
    backend: linux::LinuxPlatform,

    #[cfg(feature="state_tracker")]
    state_tracker: StateTracker,

    #[cfg(feature="post_processing")]
    post_processing: PostProcessing
}
impl Platform {
    pub fn new(external_contexts: Vec<Box<dyn ExternalContext>>) -> Self {
        #[cfg(all(target_os = "linux",feature="linux_platform"))]
        let backend = linux::LinuxPlatform::new(external_contexts).unwrap();

        #[cfg(feature="state_tracker")]
        let state_tracker = StateTracker::new();

        #[cfg(feature="post_processing")]
        let post_processing = PostProcessing::new();

        Self {
            #[cfg(feature="any_platform")]
            backend,
            #[cfg(feature="state_tracker")]
            state_tracker,
            #[cfg(feature="post_processing")]
            post_processing,
        }
    }
}

impl PlatformBackend for Platform {
    fn platform_type(&self)->PlatformType {PlatformType::Compositor}
    fn events(&mut self) -> Vec<Event> {
        #[cfg(not(feature="any_platform"))]
        let events = Vec::new();

        #[cfg(feature="any_platform")]
        let events = self.backend.events();

        #[cfg(feature="post_processing")]
        let events = self.post_processing.process(events);

        #[cfg(feature="state_tracker")]
        self.state_tracker.update(&events);

        events
    }
    fn requests(&mut self, requests: Vec<Request>) {self.backend.requests(requests);}
}

#[cfg(target_os = "linux")]
impl std::os::unix::io::AsRawFd for Platform {
    fn as_raw_fd(&self)->std::os::unix::io::RawFd {
        self.backend.as_raw_fd()
    }
}
