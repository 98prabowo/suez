mod command;
mod error;

use std::{
    collections::HashMap, 
    sync::{Arc, Mutex}
};

pub use command::Command;

pub type AtomicDB = Arc<Mutex<HashMap<String, String>>>;
