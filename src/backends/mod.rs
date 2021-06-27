pub mod graphic_backends;
pub mod input_backends;
pub mod output_backends;

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

pub(crate) mod virtual_platform;

pub struct Platform {
    #[cfg(all(target_os = "linux",feature="linux_platform"))]
    backend: linux::LinuxPlatform,

    #[cfg(feature="state_tracker")]
    state_tracker: StateTracker,

    #[cfg(feature="post_processing")]
    post_processing: PostProcessing
}
impl Platform {
    pub fn new(context: ExternalContext) -> Self {
        #[cfg(all(target_os = "linux",feature="linux_platform"))]
        let backend = linux::LinuxPlatform::new(context).unwrap();

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
    fn dispatch(&mut self) -> Vec<Event> {
        #[cfg(not(feature="any_platform"))]
        let events = Vec::new();

        #[cfg(feature="any_platform")]
        let events = self.backend.dispatch();

        #[cfg(feature="post_processing")]
        let events = self.post_processing.process(events);

        #[cfg(feature="state_tracker")]
        self.state_tracker.update(&events);

        events
    }
    fn set_keyboard_layout(&mut self, layout: String) {
        #[cfg(feature="any_platform")]
        self.backend.set_keyboard_layout(layout)
    }
    fn set_cursor_mode(&mut self, seat_id: SeatId, mode: CursorMode){}
    fn set_key_repeat(&mut self, seat_id: SeatId, value: bool){}
    fn create_surface(
        &mut self,
        output: Option<OutputId>,
    ){
        #[cfg(feature="any_platform")]
        self.backend.create_surface(output)
    }
}
