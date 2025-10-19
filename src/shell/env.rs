use super::types::{EnvVar, EnvStorage};
use crate::{ignore, ok};

use core::str::FromStr;

pub struct Environment {
    env: EnvStorage,
}

impl Environment {
    pub fn new() -> Self {
        Self { env: EnvStorage::new() }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<(), ()> {
        let key = ok!(EnvVar::from_str(key))?;
        let value = ok!(EnvVar::from_str(value))?;

        if self.env.contains_key(&key) {
            if let Some(val) = self.env.get_mut(&key) {
                *val = value;
            }
        } else {
            ignore!(self.env.insert(key, value));
        }

        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        match EnvVar::from_str(key) {
            Ok(val) => self.env.get(&val).map(|v| v.as_str()),
            Err(_)  => None,
        }
    }

    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.env.keys().into_iter().map(|k| k.as_str())
    }
}
