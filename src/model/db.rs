use std::{
    collections::HashMap, 
    sync::{Arc, Mutex}
};
use super::error::{Error, Result};

pub trait Db {
    fn insert(&self, key: String, value: String) -> Result<()>; 

    fn with_value<C>(&self, key: &str, carrier: C) -> Result<()>
    where 
        C: FnOnce(Option<&str>);
    
    fn delete(&self, key: &str) -> Result<()>; 
}

pub struct AtomicDb {
    inner: Arc<Mutex<AtomicDbInner>>,
}

struct AtomicDbInner {
    data: HashMap<String, String>,
}

impl AtomicDb {
    pub fn new() -> Self {
        Self { 
            inner: Arc::new(Mutex::new(AtomicDbInner {
                data: HashMap::new(),
            })),
        }
    }
}

impl Db for AtomicDb {
    fn insert(&self, key: String, value: String) -> Result<()> {
        let mut inner = self.inner.lock().map_err(|_| Error::DbPoisoned)?; 
        inner.data.insert(key, value);
        Ok(())
    }

    fn with_value<C>(&self, key: &str, carrier: C) -> Result<()>
    where 
        C: FnOnce(Option<&str>),
    {
        let inner = self.inner.lock().map_err(|_| Error::DbPoisoned)?; 
        carrier(inner.data.get(key).map(|value| value.as_str()));
        Ok(())
    } 

    fn delete(&self, key: &str) -> Result<()> {
        let mut inner = self.inner.lock().map_err(|_| Error::DbPoisoned)?;
        inner.data.remove(key)
            .map(|_| ())
            .ok_or(Error::KeyNotFound)
    }
}

impl Clone for AtomicDb {
    fn clone(&self) -> Self {
        Self { 
            inner: Arc::clone(&self.inner),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_value() {
        let sut = AtomicDb::new();
        let key = format!("a");
        let value = format!("data");

        sut.insert(key.clone(), value).unwrap();
        sut.with_value(&key, |val| {
            assert_eq!(val, Some("data"));
        }).unwrap();
    }

    #[test]
    fn get_value() {
        let sut = AtomicDb::new();
        let key = format!("a");
        let value = format!("data");

        sut.with_value(&key, |val| {
            assert_eq!(val, None);
        }).unwrap();

        sut.insert(key.clone(), value).unwrap();
        sut.with_value(&key, |val| {
            assert_eq!(val, Some("data"));
        }).unwrap();
    }

    #[test]
    fn delete_value() {
        let sut = AtomicDb::new();
        let key = format!("a");
        let value = format!("data");

        sut.insert(key.clone(), value).unwrap();
        sut.with_value(&key, |val| {
            assert_eq!(val, Some("data"));
        }).unwrap();

        sut.delete(&key).unwrap();
        sut.with_value(&key, |val| {
            assert_eq!(val, None);
        }).unwrap();
    }
}
