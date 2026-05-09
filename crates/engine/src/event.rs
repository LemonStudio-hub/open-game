use std::any::{Any, TypeId};
use std::collections::HashMap;

type Callback = Box<dyn FnMut(&dyn Any)>;

struct EventChannel {
    callbacks: Vec<Callback>,
}

impl EventChannel {
    fn new() -> Self {
        Self {
            callbacks: Vec::new(),
        }
    }
}

#[derive(Default)]
pub struct EventBus {
    channels: HashMap<TypeId, EventChannel>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }

    pub fn subscribe<T: 'static>(&mut self, mut callback: impl FnMut(&T) + 'static) {
        let channel = self.channels
            .entry(TypeId::of::<T>())
            .or_insert_with(EventChannel::new);

        channel.callbacks.push(Box::new(move |event: &dyn Any| {
            if let Some(e) = event.downcast_ref::<T>() {
                (callback)(e);
            }
        }));
    }

    pub fn emit<T: 'static>(&mut self, event: &T) {
        if let Some(channel) = self.channels.get_mut(&TypeId::of::<T>()) {
            for callback in &mut channel.callbacks {
                (callback)(event);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct WindowResizeEvent {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct CollisionEvent {
    pub entity_a: generational_arena::Index,
    pub entity_b: generational_arena::Index,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_bus_emit_subscribe() {
        let mut bus = EventBus::new();
        let received = std::rc::Rc::new(std::cell::Cell::new(0_i32));
        let received_clone = received.clone();

        bus.subscribe::<i32>(move |event: &i32| {
            received_clone.set(*event);
        });

        bus.emit(&42);
        assert_eq!(received.get(), 42);
    }

    #[test]
    fn test_event_bus_multiple_subscribers() {
        let mut bus = EventBus::new();
        let count = std::rc::Rc::new(std::cell::Cell::new(0));
        let c1 = count.clone();
        let c2 = count.clone();

        bus.subscribe::<i32>(move |_: &i32| { c1.set(c1.get() + 1); });
        bus.subscribe::<i32>(move |_: &i32| { c2.set(c2.get() + 1); });

        bus.emit(&1);
        assert_eq!(count.get(), 2);
    }

    #[test]
    fn test_event_bus_different_types() {
        let mut bus = EventBus::new();
        let int_val = std::rc::Rc::new(std::cell::Cell::new(0_i32));
        let str_val = std::rc::Rc::new(std::cell::RefCell::new(String::new()));

        let int_clone = int_val.clone();
        let str_clone = str_val.clone();

        bus.subscribe::<i32>(move |v| { int_clone.set(*v); });
        bus.subscribe::<String>(move |v| { *str_clone.borrow_mut() = v.clone(); });

        bus.emit(&99);
        bus.emit(&"hello".to_string());

        assert_eq!(int_val.get(), 99);
        assert_eq!(*str_val.borrow(), "hello");
    }

    #[test]
    fn test_event_bus_no_subscribers() {
        let mut bus = EventBus::new();
        bus.emit(&42_i32);
    }

    #[test]
    fn test_window_resize_event() {
        let event = WindowResizeEvent { width: 1920, height: 1080 };
        assert_eq!(event.width, 1920);
        assert_eq!(event.height, 1080);
    }
}
