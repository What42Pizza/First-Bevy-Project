use std::marker::PhantomData;



#[derive(Default)]
pub struct Watcher<T: EventSource> {
    pub last_event_id: u8,
    phantom: PhantomData<T>,
}

impl<T: EventSource> Watcher<T> {
    pub fn event_has_happened (&mut self, source: &T) -> bool {
        if self.last_event_id == source.get_id() {return false;}
        self.last_event_id = source.get_id();
        true
    }
}



pub trait EventSource {
    fn get_id (&self) -> u8;
    fn get_id_mut (&mut self) -> &mut u8;
    fn new_event (&mut self) {
        let id = self.get_id_mut();
        *id = id.wrapping_add(1);
    }
}
