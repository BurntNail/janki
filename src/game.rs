use crate::{
    either::Either,
    item::{Fact, Item, ItemGuard},
    storage::Storage,
};
use rand::{thread_rng, Rng};
use std::{collections::HashMap, time::Duration};

///Alias used to determine how long the space is between repetitions based on the current streak
pub type SeeAgainGaps = HashMap<u32, Duration>;
///Utility alias for a Vec<Item>
pub type AnkiDB = Vec<Item>;

///Struct used to manage the game - this should be used in the client
#[derive(Debug)]
pub struct AnkiGame<S: Storage> {
    ///Vector to store the items
    v: AnkiDB,
    ///Storage for the AnkiDB
    storage: S,
    ///Timer for spaced repetition
    sat: SeeAgainGaps,
}

impl<S: Storage> AnkiGame<S> {
    ///Constructor function - sets all fields to arguments, and uses the [`Storage`] to read the database.
    ///
    ///Can return [`Result::Err`] if there is an error reading the database
    pub fn new(storage: S, sat: SeeAgainGaps) -> Result<Self, S::ErrorType> {
        Ok(Self {
            v: storage.read_db()?,
            storage,
            sat,
        })
    }

    ///Gets the next card for testing from the database.
    ///
    ///Returns an [`Either`] where [`Either::Left`] signifies that the card was a random card from the cards that are eligible for testing, and the [`Either::Right`] signifies that all of the elgible cards had been tested and that a random one was picked from the whole list.
    pub fn get_card(&mut self) -> Either<ItemGuard<S>, ItemGuard<S>> {
        let elgible = get_eligible(&self.v, &self.sat);

        if elgible.is_empty() {
            let selected = thread_rng().gen_range(0..self.v.len());
            Either::Right(ItemGuard::new(&mut self.v, selected, &mut self.storage))
        } else {
            let selected = elgible[thread_rng().gen_range(0..elgible.len())];
            Either::Left(ItemGuard::new(&mut self.v, selected, &mut self.storage))
        }
    }

    ///Adds a new item to the [`AnkiDB`] using [`Into::into`] - which sets the streak to 0, and the last tested to [`Option::None`]
    pub fn add_card(&mut self, f: Fact) {
        self.v.push(f.into());
        self.storage.write_db(&self.v).unwrap();
    }
}

///A function to get all of the indexes that need to be tested from a list using a [`SeeAgainGaps`]
#[must_use]
pub(crate) fn get_eligible(items: &[Item], sat: &SeeAgainGaps) -> Vec<usize> {
    items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| {
            item.time_since_last_test()
                .map_or(Some(index), |last_seen| {
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
