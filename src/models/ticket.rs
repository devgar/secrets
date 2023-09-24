use crate::models::Dbo;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::{Error, Result};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Dbo<Ticket>>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

// CRUD Implementation
impl ModelController {
    pub async fn create_ticket(&self, ticket: Ticket) -> Result<Dbo<Ticket>> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64 + 1;
        let ticket = Dbo { id, entity: ticket };
        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Dbo<Ticket>>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn get_ticket(&self, id: u64) -> Result<Dbo<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let ticket = store.get(id as usize).and_then(|t| t.clone());

        ticket.ok_or(Error::TicketIdNotFound { id })
    }

    pub async fn delete_tickets(&self, id: u64) -> Result<Dbo<Ticket>> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketIdNotFound { id })
    }
}
