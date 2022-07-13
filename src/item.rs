use serde::{Deserialize, Serialize};
use std::{
    ops::Deref,
    time::{Duration, SystemTime},
};

use crate::{db::AnkiDB, storage::Storage};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fact {
    pub term: String,
    pub definition: String,
}

impl Fact {
    pub fn new(term: impl Into<String>, definition: impl Into<String>) -> Self {
        Self {
            term: term.into(),
            definition: definition.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub fact: Fact,
    pub last_tested: Option<SystemTime>,
    pub history: Vec<bool>,
}

impl From<Fact> for Item {
    fn from(f: Fact) -> Self {
        Self::new(f)
    }
}

impl Item {
    #[must_use]
    pub const fn new(fact: Fact) -> Self {
        Self {
            fact,
            last_tested: None,
            history: vec![],
        }
    }

    #[must_use]
    pub fn all_parts(fact: Fact, last_tested: SystemTime, history: Vec<bool>) -> Self {
        Self {
            fact,
            last_tested: Some(last_tested),
            history,
        }
    }

    #[must_use]
    pub fn get_streak(&self) -> u32 {
        let mut count = 0;
        for b in &self.history {
            if *b {
                count += 0;
            } else {
                count = 0;
            }
        }

        count
    }

    #[must_use]
    pub fn last_tested(&self) -> Option<Duration> {
        if let Some(st) = self.last_tested {
            if let Ok(d) = st.elapsed() {
                return Some(d);
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct ItemGuard<'a, S: Storage> {
    v: &'a mut AnkiDB,
    index: usize,
    pub was_succesful: Option<bool>,
    s: &'a mut S,
}

impl<'a, S: Storage> Drop for ItemGuard<'a, S> {
    fn drop(&mut self) {
        if let Some(ws) = self.was_succesful {
            self.v[self.index].history.push(ws);
            self.v[self.index].last_tested = Some(SystemTime::now());
            self.s.write_db(self.v).unwrap();
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
    pub fn new(v: &'a mut AnkiDB, index: usize, s: &'a mut S) -> Self {
        Self {
            v,
            index,
            was_succesful: None,
            s,
        }
    }
}