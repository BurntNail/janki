use crate::{
    item::{Fact, Item, ItemGuard},
    storage::Storage,
};
use rand::{thread_rng, Rng};
use std::{collections::HashMap, time::Duration};

pub type SeeAgainTimer = HashMap<u32, Duration>;
pub type AnkiDB = Vec<Item>;

#[derive(Debug)]
pub struct AnkiGame<S: Storage> {
    v: Vec<Item>,
    storage: S,
    sat: SeeAgainTimer,
}

impl<S: Storage> AnkiGame<S> {
    pub fn new(mut storage: S, sat: SeeAgainTimer) -> Result<Self, S::ErrorType> {
        Ok(Self {
            v: storage.read_db()?,
            storage,
            sat,
        })
    }

    pub fn get_card(&mut self) -> ItemGuard<S> {
        let elgible = get_elgible(&self.v, &self.sat);
        
        let selected = if elgible.is_empty() {
            thread_rng().gen_range(0..self.v.len())
        } else {
            elgible[thread_rng().gen_range(0..elgible.len())]
        };

        ItemGuard::new(&mut self.v, selected, &mut self.storage)
    }

    pub fn add_card(&mut self, f: Fact) {
        self.v.push(f.into());
        self.storage.write_db(&self.v).unwrap();
    }
}

fn get_elgible(items: &[Item], sat: &SeeAgainTimer) -> Vec<usize> {
    items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| {
            item.time_since_last_test().map_or(Some(index), |last_seen| {
                sat.get(&item.get_streak()).map_or(Some(index), |distance| {
                    if &last_seen > distance {
                        Some(index)
                    } else {
                        None
                    }
                })
            })
        })
        .collect()
}
