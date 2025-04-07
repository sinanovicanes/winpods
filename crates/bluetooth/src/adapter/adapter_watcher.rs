use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

use utils::EventDispatcher;

use crate::{AdapterState, get_adapter_state};

struct AdapterStateChangedEvent(AdapterState);

pub struct AdapterWatcher {
    dispatcher: EventDispatcher,
    thread_handle: Option<JoinHandle<()>>,
}

impl AdapterWatcher {
    pub fn new() -> Self {
        let dispatcher = EventDispatcher::new();

        AdapterWatcher {
            dispatcher,
            thread_handle: None,
        }
    }

    pub fn start(&mut self) {
        if self.thread_handle.is_some() {
            return;
        }

        let mut current_state = get_adapter_state();
        let dispatcher_clone = self.dispatcher.clone();
        let thread_handle = thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));

                let new_state = get_adapter_state();

                if new_state != current_state {
                    current_state = new_state;
                    dispatcher_clone.dispatch(AdapterStateChangedEvent(new_state));
                }
            }
        });

        self.thread_handle = Some(thread_handle);
    }

    pub fn on_state_changed(&self, callback: impl Fn(AdapterState) + Send + Sync + 'static) {
        self.dispatcher
            .add_listener::<AdapterStateChangedEvent, _>(move |event| {
                callback(event.0);
            });
    }
}
