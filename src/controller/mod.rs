use std::{
    io::{BufRead, BufReader, Write}, 
    net::TcpStream, 
};

use crate::model::{AtomicDB, Command};

pub struct Controller;

impl Controller {
    pub fn handle_input(
        stream: &mut TcpStream, 
        db: AtomicDB
    ) {
        let buf_reader = BufReader::new(&mut *stream);

        if let Some(Ok(request_line)) = buf_reader.lines().next() {
            let respond: String = Self::handle_cmd(db, request_line);
            if let Err(error) = stream.write_all(respond.as_bytes()) {
                println!("{:<2} - fail send response. Reason: {}", "TCP", error);
            }
        }
    }

    fn handle_cmd(
        db: AtomicDB,
        request_line: String
    ) -> String {
        match request_line.parse() {
            Ok(Command::GET(key))           => Controller::handle_get(db, key),
            Ok(Command::SET{ key, value })  => Controller::handle_set(db, key, value),
            Ok(Command::DELETE(key))        => Controller::handle_del(db, key),
            Err(error)                      => format!("{}\n", error),
        }
    }

    fn handle_get(
        db: AtomicDB,
        key: String
    ) -> String {
        match db.lock() {
            Ok(db) => {
                if let Some(value) = db.get(&key) {
                    format!("{}\n", value)
                } else {
                    format!("No value for key: {}\n", key)
                }
            }
            Err(_) => format!("Failed to get data\n"),
        }
    }

    fn handle_set(
        db: AtomicDB,
        key: String,
        value: String
    ) -> String {
        match db.lock() {
            Ok(mut db) => {
                if let Some(current_v) = db.get_mut(&key) {
                    *current_v = value
                } else { 
                    db.entry(key).or_insert(value);
                }
                "Success entry data\n".into()
            }
            Err(_) => "Failed to entry data\n".into(),
        }
    }

    fn handle_del(
        db: AtomicDB,
        key: String
    ) -> String {
        match db.lock() {
            Ok(mut db) => {
                if let Some(_) = db.remove(&key) {
                    format!("Success remove {}\n", key)
                } else {
                    format!("No value for key: {}\n", key)
                }
            }
            Err(_) => "Failed to delete data\n".into(),
        }
    }
}

#[cfg(test)]
mod controller_tests {
    use std::{collections::HashMap, sync::{Arc, Mutex}};

    use super::*;

    #[test]
    fn get_handler_test() {
        let (sut, key) = make_sut();
        let response: String = Controller::handle_get(sut, key);
        assert_eq!(response, "No value for key: abc\n");
    }

    #[test]
    fn set_handler_test() {
        let (sut, key) = make_sut();
        let _: String = Controller::handle_set(Arc::clone(&sut), key.clone(), "10".into());
        let response: String = Controller::handle_get(Arc::clone(&sut), key.clone());
        assert_eq!(response, "10\n");
    }

    #[test]
    fn del_handler_test() {
        let (sut, key) = make_sut();

        let _set_value: String = Controller::handle_set(Arc::clone(&sut), key.clone(), "10".into());
        let response: String = Controller::handle_get(Arc::clone(&sut), key.clone());
        assert_eq!(response, "10\n");

        let _delete_value: String = Controller::handle_del(Arc::clone(&sut), key.clone());
        let response: String = Controller::handle_get(Arc::clone(&sut), key.clone());
        assert_eq!(response, "No value for key: abc\n");
    }

    #[test]
    fn input_handler_with_unknown_cmd_test() {
        let (sut, request_line) = make_sut();
        let response: String = Controller::handle_cmd(sut, request_line);
        assert_eq!(response, "Unknown command\n");
    }

    // Helpers

    fn make_sut() -> (AtomicDB, String) {
        let sut: AtomicDB = Arc::new(Mutex::new(HashMap::new()));
        let key: String = String::from("abc");
        (sut, key)
    }
}
