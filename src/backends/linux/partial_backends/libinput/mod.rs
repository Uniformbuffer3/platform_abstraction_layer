

pub struct LibinputBackend {

}
/*
impl LibinputBackend {
    pub fn new() -> Self {
        let context = input::Libinput::new_with_udev(Interface);
        let keystroke_decoder = KeystrokeDecoder::new();
        let id_counter = 0;
        Self {
            context,
            keystroke_decoder,
            id_counter,
        }
    }

    pub fn dispatch(&mut self) -> Vec<input::event::Event> {
        self.context.dispatch().unwrap();
        let mut events = Vec::new();
        while let Some(event) = self.context.next() {events.push(event);}
        events
    }

    fn set_keyboard_layout(&mut self, layout: String) {
        self.keystroke_decoder.set_layout(layout);
    }
}*/
