use educe::Educe;
use tap::Pipe;

use core::fmt;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{action::Action, client::Client, server::Addr};

#[derive(Clone, Educe)]
#[educe(Default(new))]
pub struct Lobby {
    pub clients: Arc<Mutex<HashMap<Addr, Client>>>,
}

impl fmt::Display for Lobby {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (addr, client) in self.clients.lock().expect("Poisoned Mutex").iter() {
            writeln!(f, "{}: {}", addr, client)?;
        }

        Ok(())
    }
}

impl Lobby {
    pub fn insert(&self, addr: Addr, client: Client) -> Option<Client> {
        self.clients
            .lock()
            .expect("Poisoned Mutex")
            .insert(addr, client)
    }

    pub fn remove(&self, addr: &str) -> Option<Client> {
        self.clients.lock().expect("Poisoned Mutex").remove(addr)
    }

    pub fn update(&self, addr: &str, action: Action) -> Option<()> {
        self.clients
            .lock()
            .expect("Poisoned Mutex")
            .get_mut(addr)?
            .pipe(|client| client.update(action));

        Some(())
    }
}
