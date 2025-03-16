use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A type-erased event listener that can be stored in the dispatcher
#[allow(dead_code)]
trait EventListener: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn call(&self, event: &dyn Any);
}

/// A strongly-typed event listener implementation
struct TypedEventListener<E, F>
where
    E: 'static + Send + Sync,
    F: Fn(&E) + Send + Sync + 'static,
{
    callback: F,
    _phantom: std::marker::PhantomData<E>,
}

impl<E, F> TypedEventListener<E, F>
where
    E: 'static + Send + Sync,
    F: Fn(&E) + Send + Sync + 'static,
{
    fn new(callback: F) -> Self {
        Self {
            callback,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<E, F> EventListener for TypedEventListener<E, F>
where
    E: 'static + Send + Sync,
    F: Fn(&E) + Send + Sync + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn call(&self, event: &dyn Any) {
        if let Some(typed_event) = event.downcast_ref::<E>() {
            (self.callback)(typed_event);
        }
    }
}

type EventListenerMap = HashMap<TypeId, Vec<Arc<dyn EventListener>>>;

/// Event dispatcher that allows registering listeners and dispatching events
pub struct EventDispatcher {
    listeners: Arc<Mutex<EventListenerMap>>,
}

impl EventDispatcher {
    /// Create a new event dispatcher
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a listener for a specific event type
    pub fn add_listener<E, F>(&self, callback: F)
    where
        E: 'static + Send + Sync,
        F: Fn(&E) + Send + Sync + 'static,
    {
        let listener = Arc::new(TypedEventListener::new(callback));
        let type_id = TypeId::of::<E>();

        let mut listeners = self.listeners.lock().unwrap();
        listeners.entry(type_id).or_default().push(listener);
    }

    /// Dispatch an event to all registered listeners for its type
    pub fn dispatch<E>(&self, event: E)
    where
        E: 'static + Send + Sync,
    {
        let type_id = TypeId::of::<E>();
        let listeners = self.listeners.lock().unwrap();

        if let Some(event_listeners) = listeners.get(&type_id) {
            for listener in event_listeners {
                listener.call(&event);
            }
        }
    }

    /// Remove all listeners for a specific event type
    pub fn clear_listeners<E>(&self)
    where
        E: 'static,
    {
        let type_id = TypeId::of::<E>();
        let mut listeners = self.listeners.lock().unwrap();
        listeners.remove(&type_id);
    }

    /// Remove all listeners for all event types
    pub fn clear_all_listeners(&self) {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.clear();
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for EventDispatcher {
    fn clone(&self) -> Self {
        Self {
            listeners: Arc::clone(&self.listeners),
        }
    }
}
