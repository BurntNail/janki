use crate::{game::AnkiDB, storage::Storage};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Deref};

#[cfg(feature = "druid_data")]
use druid::Data;

///A Fact - a term and a definition
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "druid_data", derive(Data))]
pub struct Fact {
    ///The term of the fact - this is given to the test taker
    pub term: String,
    ///The definition of the fact - the test taker gives this.
    pub definition: String,
}

impl Fact {
    ///Fact constructor using [`Into`]
    pub fn new(term: impl Into<String>, definition: impl Into<String>) -> Self {
        Self {
            term: term.into(),
            definition: definition.into(),
        }
    }
}

impl Display for Fact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Fact: {:?} == {:?}", self.term, self.definition)
    }
}

///An Item - contains a fact, as well as stats about the user's history with that fact.
///
///Often accessed in the client via an [`ItemGuard`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    ///The fact that is the focus of the item
    pub fact: Fact,
    ///The last time the user saw this fact.
    ///
    ///Can be [`Option::None`] if the user has never been tested on this before.
    ///
    ///Clients should never directly access this, as this is set via an [`ItemGuard`] or otherwise
    pub(crate) last_tested: Option<DateTime<Utc>>,
    ///The history of the user - each bool signifies whether or not the user answered correctly.
    ///
    ///`history[0]` is the first time that the user was tested on the fact, and as the user is tested again, `history.push` is used.
    ///
    ///Clients should never directly access this, as this is set via an [`ItemGuard`] or otherwise
    pub(crate) history: Vec<bool>,
}

impl From<Fact> for Item {
    fn from(f: Fact) -> Self {
        Self::new(f)
    }
}
impl From<Item> for Fact {
    fn from(i: Item) -> Self {
        i.fact
    }
}

impl Item {
    ///Constructor for a new [`Item`] - sets the `last_tested` to [`Option::None`] and the `history` to an empty `Vec<bool>`
    #[must_use]
    pub(crate) const fn new(fact: Fact) -> Self {
        Self {
            fact,
            last_tested: None,
            history: vec![],
        }
    }

    ///Constructor for a new [`Item`] where all fields are given as arguments
    #[must_use]
    #[allow(dead_code)]
    pub(crate) const fn all_parts(
        fact: Fact,
        last_tested: DateTime<Utc>,
        history: Vec<bool>,
    ) -> Self {
        Self {
            fact,
            last_tested: Some(last_tested),
            history,
        }
    }

    ///Gets the user's streak for that fact - AKA the number of times in a row that they have answered correctly, with a correction factor to not make the user start from beginning on every mistake.
    #[must_use]
    pub fn get_streak(&self) -> u32 {
        let min = if self.history.contains(&true) && self.true_streak() > 0 {
            1
        } else {
            0
        };

        let mut count = 0;
        for b in &self.history {
            if *b {
                count += 1;
            } else {
                count /= 2;
            }
        }

        count.min(min)
    }

    ///Gets the user's streak - the number of times they have correctly answered in a row
    pub(crate) fn true_streak(&self) -> u32 {
        let mut count = 0;
        for b in &self.history {
            if *b {
                count += 1;
            } else {
                count = 0;
            }
        }

        count
    }

    ///Gets the time since the user was last tested on this fact.
    ///
    ///Can return a [`None`] if the user was never tested, or was tested in the future due to a [`SystemTime`] error
    #[must_use]
    #[instrument(skip(self))]
    pub fn time_since_last_test(&self) -> Option<Duration> {
        if let Some(st) = self.last_tested {
            let diff = Utc::now() - st;
            let zero = Duration::zero();

            if diff.max(zero) != zero {
                return Some(diff);
            } else {
                error!("Negative Time... {}", diff);
            }
        }

        None
    }
}

///Guard for [`Item`] for Client use.
///
///On [`Drop::drop`], the [`crate::game::AnkiGame`] is updated, and as of such only one [`ItemGuard`] can exist per [`crate::game::AnkiGame`]
#[derive(Debug)] //TODO: refactor for concurrency
pub struct ItemGuard<'a, S: Storage> {
    ///A mutable reference to the [`AnkiDB`] from the [`crate::game::AnkiGame`]
    v: &'a mut AnkiDB,
    ///The index in the database for the item.
    index: usize,
    ///A mutable reference to the [`Storage`] for the [`crate::game::AnkiGame`]
    s: &'a mut S,

    ///Whether or not the user was correct.
    ///
    ///This should start as an [`Option::None`], and then be changed to `Some(true)` or `Some(false)` when the user answers.
    pub was_succesful: Option<bool>,
}

impl<'a, S: Storage> Drop for ItemGuard<'a, S> {
    ///On drop, assuming the question was answered (AKA `self.was_successful.is_some()`), the following happens:
    ///
    /// - the `history` and `last_tested` of the underlying item are updated.
    /// - the database is written using [`Storage::write_db`]
    fn drop(&mut self) {
        if let Some(ws) = self.was_succesful {
            if let Some(el) = self.v.get_mut(self.index) {
                el.history.push(ws);
                el.last_tested = Some(Utc::now());
                self.s.write_db(self.v).unwrap();

                //TODO: ability to invalidate an IG
            }
        }
    }
}

impl<'a, S: Storage> Deref for ItemGuard<'a, S> {
    type Target = Fact;

    fn deref(&self) -> &Self::Target {
        &self.v[self.index].fact
    }
}

impl<'a, S: Storage> ItemGuard<'a, S> {
    ///Constructor for a new [`ItemGuard`] - should only be called by an [`crate::game::AnkiGame`]
    pub(crate) fn new(v: &'a mut AnkiDB, index: usize, s: &'a mut S) -> Self {
        Self {
            v,
            index,
            was_succesful: None,
            s,
        }
    }
}
