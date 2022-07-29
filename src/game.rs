use crate::{
    dummy_storage::{DummyStorage, DynStorage},
    item::{Fact, Item, ItemGuard},
    storage::Storage,
};
use rand::{thread_rng, Rng};
use std::{
    collections::HashMap,
    marker::PhantomData,
};
use tracing::Level;
use chrono::{Duration};
use chrono::Utc;

///Alias used to determine how long the space is between repetitions based on the current streak
pub type SeeAgainGaps = HashMap<u32, Duration>;
///Utility alias for a Vec<Item>
pub type AnkiDB = Vec<Item>;

///muah ha ha ha ha
///
///Library users can't see this, so I can do real messed up stuff with the orphans
mod private_trait {
    ///get recked lol
    pub trait CannotExternallyImplement {}
}

///Trait for clients to decide how they want to get the facts. Shenanigans have occured so that clients **cannot** implement this
pub trait AnkiCardReturnType: private_trait::CannotExternallyImplement {}
///Marker Struct that implements [`AnkiCardReturnType`] where the client receives an [`ItemGuard`]
pub struct GiveItemGuards;
///Marker Struct that implements [`AnkiCardReturnType`] where the client receives a [`Fact`]
pub struct GiveFacts;
impl private_trait::CannotExternallyImplement for GiveItemGuards {}
impl private_trait::CannotExternallyImplement for GiveFacts {}
impl AnkiCardReturnType for GiveItemGuards {}
impl AnkiCardReturnType for GiveFacts {}

///Provides a default [`SeeAgainGaps`] - useful for testing
#[must_use]
pub fn default_sag() -> SeeAgainGaps {
    let mut hm = HashMap::new();
    for i in 1..11 {
        hm.insert(i, Duration::seconds(i64::from(i) * 30));
    }
    hm
}

///Struct used to manage the game - this should be used in the client
pub struct AnkiGame<S: Storage, T: AnkiCardReturnType> {
    ///Vector to store the items
    v: AnkiDB,
    ///Storage for the AnkiDB
    pub(crate) storage: S,
    ///Timer for spaced repetition
    sag: SeeAgainGaps,
    ///Stores the index of the card being tested if [`AnkiCardReturnType`] == [`GiveFacts`]
    current: Option<(usize, bool)>,

    ///Makes sure that the [`AnkiCardReturnType`] isn't optimised away
    _pd: PhantomData<T>,
}

impl<S: Storage, T: AnkiCardReturnType> AnkiGame<S, T> {
    ///Constructor function - sets all fields to arguments, and uses the [`Storage`] to read the database.
    ///
    ///Can return [`Result::Err`] if there is an error reading the database
    pub fn new(storage: S, sag: SeeAgainGaps) -> Result<Self, S::ErrorType> {
        Ok(Self {
            v: storage.read_db()?,
            storage,
            sag,
            current: None,
            _pd: PhantomData,
        })
    }

    ///Constructor from parameters
    pub fn new_params(v: AnkiDB, storage: S, sag: SeeAgainGaps) -> Self {
        Self {
            v,
            storage,
            sag,
            current: None,
            _pd: PhantomData,
        }
    }

    ///Adds a new item to the [`AnkiDB`] using [`Into::into`] - which sets the streak to 0, and the last tested to [`Option::None`]
    pub fn add_fact(&mut self, f: Fact) {
        trace!("New fact - {f:?}");
        self.v.push(f.into());
        self.storage.write_db(&self.v).unwrap();
    }

    ///Gets all the current eligible facts - the ordering is **not** related to anything
    #[must_use]
    pub fn get_eligible(&self) -> Vec<Fact> {
        let indices = get_eligible(&self.v, &self.sag);
        indices
            .into_iter()
            .map(|index| &self.v[index].fact)
            .cloned()
            .collect()
    }

    ///Get the number of facts in the eligible list
    #[must_use]
    pub fn get_eligible_no(&self) -> usize {
        get_eligible(&self.v, &self.sag).len()
    }

    ///Gets **all** of the current facts
    #[must_use]
    pub fn get_all_facts(&self) -> Vec<Fact> {
        self.v.clone().into_iter().map(Into::into).collect()
    }

    ///Deletes a fact at a given index
    #[instrument(skip(self))]
    pub fn delete_at_index(&mut self, index: usize) {
        if self.v.len() > index {
            self.v.remove(index);
        }
    }

    ///Adds a list of facts to the database
    pub fn add_facts(&mut self, v: Vec<Fact>) {
        let current_facts = self.get_all_facts();
        for f in v {
            if !current_facts.contains(&f) {
                self.add_fact(f);
            }
        }
    }

    ///Writes to the database - useful if the function is called externally, like in eframe
    pub fn write_to_db(&mut self) -> Result<(), S::ErrorType> {
        self.storage.write_db(&self.v)
    }

    ///Gets an index for use in a [`get_new_card`] or [`get_fact`]
    ///
    ///Returns the index to use and a bool for whether the item was taken from the eligible list
    fn get_an_index(&self) -> Option<(usize, bool)> {
        let eligible = get_eligible(&self.v, &self.sag);

        if eligible.is_empty() {
            if self.v.is_empty() {
                None
            } else {
                Some((thread_rng().gen_range(0..self.v.len()), false))
            }
        } else {
            Some((eligible[thread_rng().gen_range(0..eligible.len())], true))
        }
    }
}

impl<S: Storage> AnkiGame<S, GiveItemGuards> {
    ///Gets a new card from the eligible list. If there are no terms, it will return [`Option::None`].
    ///
    ///Returns an [`ItemGuard`] and a [`bool`] for whether the item was taken from the eligible list
    pub fn get_item_guard(&mut self) -> Option<(ItemGuard<S>, bool)> {
        if let Some((index, was_e)) = self.get_an_index() {
            Some((ItemGuard::new(&mut self.v, index, &mut self.storage), was_e))
        } else {
            None
        }
    }

    ///Sets the current [`AnkiCardReturnType`] to be [`GiveFacts`] over [`GiveItemGuards`]
    pub fn to_give_facts(self) -> AnkiGame<S, GiveFacts> {
        AnkiGame::new_params(self.v, self.storage, self.sag)
    }

    ///Function to clean everything up for exit
    #[instrument(skip(self))]
    pub fn exit(&mut self) {
        trace!("Calling exit");
        self.storage.exit_application();
    }

    ///Clears **all** items from the [`AnkiDB`]
    pub fn clear(&mut self) {
        self.v.clear();
    }
}

impl<S: Storage> AnkiGame<S, GiveFacts> {
    ///Gets a fact.
    ///
    ///If no facts, will return [`Option::None`], else will return a [`Fact`] and a [`bool`] for whether or not is was from the eligible list
    pub fn get_fact(&mut self) -> Option<(Fact, bool)> {
        if let Some((cu, was_e)) = self.current {
            Some((self.v[cu].fact.clone(), was_e))
        } else if self.v.is_empty() {
            None
        } else {
            self.set_new_fact();
            self.get_fact()
        }
    }

    ///Combination of [`Self::set_new_fact`] and [`Self::get_fact`] - to ensure that the fact received is new
    pub fn get_new_fact(&mut self) -> Option<(Fact, bool)> {
        self.set_new_fact();
        self.get_fact()
    }

    ///Sets the current fact to a new fact
    #[instrument(skip(self))]
    pub fn set_new_fact(&mut self) {
        if let Some((index, we)) = self.get_an_index() {
            event!(Level::INFO, index, we, "Setting new fact");
            self.current = Some((index, we));
        } else {
            warn!("Unable to get a new index");
        }
    }

    ///Signifies that the client is done with the fact.
    #[instrument(skip(self))]
    pub fn finish_current_fact(&mut self, correct: Option<bool>) {
        trace!("Finishing current fact");

        if let Some((cu, _)) = self.current {
            if let Some(correct) = correct {
                event!(Level::INFO, cu, correct, "Finishing current fact");

                self.v[cu].history.push(correct);
                self.v[cu].last_tested = Some(Utc::now());
                self.storage
                    .write_db(&self.v)
                    .expect("unable to write to db");
            } else {
                event!(Level::WARN, cu, "Correct not marked");
            }
        }

        self.current = None;
    }

    ///Sets the current [`AnkiCardReturnType`] to be [`GiveItemGuards`] over [`GiveFacts`]
    pub fn to_give_item_guards(self) -> AnkiGame<S, GiveItemGuards> {
        AnkiGame::new_params(self.v, self.storage, self.sag)
    }

    ///Function to clean everything up for exit
    #[instrument(skip(self))]
    pub fn exit(&mut self) {
        trace!("Calling exit");
        self.finish_current_fact(None);
        self.storage.exit_application();
    }

    ///Clears **all** items from the [`AnkiDB`]
    pub fn clear(&mut self) {
        self.finish_current_fact(None);
        self.v.clear();
    }
}

impl<E: std::fmt::Debug, T: AnkiCardReturnType> DynStorage<E> for AnkiGame<DummyStorage, T> {
    fn read_custom(&mut self, s: &dyn Storage<ErrorType = E>) -> Result<(), E> {
        self.v = s.read_db()?;
        Ok(())
    }

    fn write_custom(&mut self, s: &mut dyn Storage<ErrorType = E>) -> Result<(), E> {
        s.write_db(&self.v)
    }

    fn exit_custom(&mut self, s: &mut dyn Storage<ErrorType = E>) {
        s.exit_application();
    }
}

///A function to get all of the indexes that need to be tested from a list using a [`SeeAgainGaps`]
#[must_use]
pub fn get_eligible(items: &[Item], sag: &SeeAgainGaps) -> Vec<usize> {
    items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| {
            item.time_since_last_test()
                .map_or(Some(index), |last_seen| {
                    sag.get(&item.get_streak()).map_or(Some(index), |distance| {
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
