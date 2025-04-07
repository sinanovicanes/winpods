use utils::EventDispatcher;
use windows::{Devices::Radios::Radio, Foundation::TypedEventHandler};

use super::get_bluetooth_adapter_radio;
use crate::{AdapterState, get_adapter_state};

struct AdapterStateChangedEvent(AdapterState);

pub struct AdapterWatcher {
    dispatcher: EventDispatcher,
    radio: Option<Radio>,
}

impl AdapterWatcher {
    pub fn new() -> Self {
        let dispatcher = EventDispatcher::new();

        AdapterWatcher {
            dispatcher,
            radio: None,
        }
    }

    pub fn state(&self) -> AdapterState {
        match self.radio {
            Some(ref radio) => AdapterState::from(radio),
            None => get_adapter_state(),
        }
    }

    pub fn start(&mut self) {
        let Some(radio) = get_bluetooth_adapter_radio() else {
            return;
        };

        let mut current_state = AdapterState::from(&radio);
        let dispatcher = self.dispatcher.clone();
        let _ = radio.StateChanged(&TypedEventHandler::<Radio, _>::new(move |radio, _| {
            let Some(radio) = radio.as_ref() else {
                return Ok(());
            };

            let new_state = AdapterState::from(radio);

            if new_state == current_state {
                return Ok(());
            }

            current_state = new_state;
            dispatcher.dispatch(AdapterStateChangedEvent(new_state));
            Ok(())
        }));

        self.radio = Some(radio);
    }

    pub fn stop(&mut self) {
        self.radio = None;
    }

    pub fn on_state_changed(&self, callback: impl Fn(AdapterState) + Send + Sync + 'static) {
        self.dispatcher
            .add_listener::<AdapterStateChangedEvent, _>(move |event| {
                callback(event.0);
            });
    }
}
