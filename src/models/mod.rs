pub mod ticket;
pub mod user;

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dbo<T: Serialize> {
    pub id: u64,
    #[serde(flatten)]
    entity: T,
}

pub trait DboTable {
    fn get_table() -> &'static str;
}

impl<T: Serialize + Default + DboTable> Dbo<T> {
    pub fn insert(payload: T) -> Self {
        let mut rng = rand::thread_rng();
        tracing::info!("DB inserting data into {}", T::get_table());
        // DO SOMETHING TO INSERT IN DATABASE
        Self {
            id: rng.gen_range(1..999),
            entity: payload,
        }
    }

    pub fn get(id: u64) -> Self {
        Self {
            id,
            entity: T::default(),
        }
    }
}
