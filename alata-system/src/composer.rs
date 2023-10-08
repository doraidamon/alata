use std::any::TypeId;

pub enum ComposerEvent {
    ValueChanged,
}

pub struct ComposerEventTrigger {
    pub type_id: TypeId,
    pub event_type: ComposerEvent
}

pub trait Composer<E> where Self: 'static {
    fn recieve_event(&mut self, event: E);
    
    /// 
    fn when(event_type: ComposerEvent) -> ComposerEventTrigger {
        ComposerEventTrigger {
            type_id: TypeId::of::<Self>(),
            event_type
        }
    }
}
