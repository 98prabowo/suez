use std::{
    io::{BufRead, BufReader, Write}, 
    net::TcpStream, 
};

use crate::model::{Command, Db, error::Error};

pub struct Controller;

impl Controller {
    pub fn handle_input(
        stream: &mut TcpStream, 
        db: impl Db 
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
        db: impl Db,
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
        db: impl Db,
        key: String
    ) -> String {
        let mut value: Option<String> = None; 

        match db.with_value(&key, |val| { 
            value = val.map(|v| format!("{}\n", v) ); 
        }) {
            Ok(_)   => value.unwrap_or(format!("No value for key: {}\n", key)),
            Err(_)  => format!("Failed to get data\n"),
        }
    }

    fn handle_set(
        db: impl Db,
        key: String,
        value: String
    ) -> String {
        match db.insert(key, value) {
            Ok(_)   => format!("Success entry data\n"),
            Err(_)  => format!("Failed to entry data\n"),
        }
    }

    fn handle_del(
        db: impl Db,
        key: String
    ) -> String {
        match db.delete(&key) {
            Ok(_)                   => format!("Success remove {}\n", key),
            Err(Error::KeyNotFound) => format!("No value for key: {}\n", key),
            Err(_)                  => format!("Failed to delete data\n"),

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::error::Result;

    #[test]
    fn fail_get_value() {
        let (sut, key) = make_sut();
        let response: String = Controller::handle_get(sut, key);
        assert_eq!(response, "Failed to get data\n");
    }

    #[test]
    fn get_none_value() {
        let (mut sut, key) = make_sut();
        sut.set_value_action(Ok(None));
        let response: String = Controller::handle_get(sut, key);
        assert_eq!(response, "No value for key: abc\n");
    }

    #[test]
    fn get_value() {
        let (mut sut, key) = make_sut();
        sut.set_value_action(Ok(Some("10")));
        let response: String = Controller::handle_get(sut, key);
        assert_eq!(response, "10\n");
    }

    #[test]
    fn fail_set_value() {
        let (sut, key) = make_sut();
        let response: String = Controller::handle_set(sut.clone(), key.clone(), "10".into());
        assert_eq!(response, "Failed to entry data\n");
    }

    #[test]
    fn set_value() {
        let (mut sut, key) = make_sut();
        sut.set_insert_action(Ok(()));
        let response: String = Controller::handle_set(sut.clone(), key.clone(), "10".into());
        assert_eq!(response, "Success entry data\n");
    }

    #[test]
    fn fail_del_value() {
        let (sut, key) = make_sut();
        let response: String = Controller::handle_del(sut.clone(), key.clone());
        assert_eq!(response, "Failed to delete data\n");
    }

    #[test]
    fn del_none_value() {
        let (mut sut, key) = make_sut();
        sut.set_delete_action(Err(Error::KeyNotFound));
        let response: String = Controller::handle_del(sut.clone(), key.clone());
        assert_eq!(response, "No value for key: abc\n");
    }

    #[test]
    fn del_value() {
        let (mut sut, key) = make_sut();
        sut.set_delete_action(Ok(()));
        let response: String = Controller::handle_del(sut.clone(), key.clone());
        assert_eq!(response, "Success remove abc\n");
    }

    #[test]
    fn input_handler_with_unknown_cmd_test() {
        let (sut, request_line) = make_sut();
        let response: String = Controller::handle_cmd(sut, request_line);
        assert_eq!(response, "Unknown command\n");
    }

    // Helpers
    
    #[derive(Debug, Clone)]
    struct MockDb<'a> {
        value_action: Result<Option<&'a str>>,
        insert_action: Result<()>,
        delete_action: Result<()>,
    }

    impl<'a> MockDb<'a> {
        fn new() -> Self {
            Self { 
                value_action: Err(Error::UnknownCommand), 
                insert_action: Err(Error::UnknownCommand), 
                delete_action: Err(Error::UnknownCommand),
            }
        }

        fn set_value_action(&mut self, action: Result<Option<&'a str>>) {
            self.value_action = action;
        }

        fn set_insert_action(&mut self, action: Result<()>) {
            self.insert_action= action;
        }

        fn set_delete_action(&mut self, action: Result<()>) {
            self.delete_action = action;
        }
    }

    impl<'a> Db for MockDb<'a> {
        fn insert(&self, _: String, _: String) -> Result<()> {
            self.insert_action.clone()
        }

        fn with_value<C>(&self, _: &str, carrier: C) -> Result<()>
        where 
            C: FnOnce(Option<&'a str>) 
        {
            match &self.value_action {
                Ok(value) => {
                    carrier(*value);
                    Ok(())
                },
                Err(e) => Err(e.clone()),
            }
        }

        fn delete(&self, _: &str) -> Result<()> {
            self.delete_action.clone()
        }
    }

    fn make_sut<'a>() -> (MockDb<'a>, String) {
        let sut = MockDb::new();
        let key: String = String::from("abc");
        (sut, key)
    }
}
