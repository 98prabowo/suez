use std::str::FromStr;

use super::error::Error;

#[derive(Debug, PartialEq)]
pub enum Command {
    GET(String),
    SET{ key: String, value: String },
    DELETE(String),
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(cmd: &str) -> std::result::Result<Self, Self::Err> {
        let cmd_lines: Vec<&str> = cmd.split_whitespace().collect();

        if cmd_lines.len() <= 1 && cmd_lines.len() > 3 {
            return Err(Error::UnknownCommand)
        }

        let cmd = cmd_lines.get(0).ok_or(Error::UnknownCommand)?;
        let key = cmd_lines.get(1);
        let value = cmd_lines.get(2);

        match cmd.to_lowercase().as_str() {
            "get" => Ok(Self::GET(key.ok_or(Error::KeyNotFound)?.to_string())),
            "set" => Ok(Self::SET { 
                key: key.ok_or(Error::KeyNotFound)?.to_string(), 
                value: value.ok_or(Error::ValueNotFound)?.to_string(), 
            }),
            "del" | "delete" => Ok(Self::DELETE(key.ok_or(Error::KeyNotFound)?.to_string())),
            _ => Err(Error::UnknownCommand),
        }
    }
}

#[cfg(test)]
mod command_tests {
    use super::*;

    #[test]
    fn empty_cmd_test() {
        let sut: String = String::from("");
        assert_eq!(sut.parse::<Command>(), Err(Error::UnknownCommand));
    }

    #[test]
    fn unknown_cmd_test() {
        let sut: String = String::from("gibberish");
        assert_eq!(sut.parse::<Command>(), Err(Error::UnknownCommand));
    }

    #[test]
    fn get_cmd_test() {
        let sut: String = String::from("get abc");
        assert_eq!(sut.parse(), Ok(Command::GET("abc".into())));
    }

    #[test]
    fn get_cmd_no_key_test() {
        let sut: String = String::from("get");
        assert_eq!(sut.parse::<Command>(), Err(Error::KeyNotFound)); 
    }

    #[test]
    fn get_cmd_with_value_test() {
        let sut: String = String::from("get abc 10");
        assert_eq!(sut.parse(), Ok(Command::GET("abc".into())));
    }

    #[test]
    fn set_cmd_test() {
        let sut: String = String::from("set abc 10");
        assert_eq!(sut.parse(), Ok(Command::SET { key: "abc".into(), value: "10".into() }));
    }

    #[test]
    fn set_cmd_no_key_test() {
        let sut: String = String::from("set");
        assert_eq!(sut.parse::<Command>(), Err(Error::KeyNotFound)); 
    }

    #[test]
    fn set_cmd_no_value_test() {
        let sut: String = String::from("set abc");
        assert_eq!(sut.parse::<Command>(), Err(Error::ValueNotFound)); 
    }

    #[test]
    fn del_cmd_test() {
        let sut: String = String::from("del abc");
        assert_eq!(sut.parse(), Ok(Command::DELETE("abc".into())));
    }

    #[test]
    fn del_cmd_no_key_test() {
        let sut: String = String::from("del");
        assert_eq!(sut.parse::<Command>(), Err(Error::KeyNotFound)); 
    }

    #[test]
    fn delete_cmd_test() {
        let sut: String = String::from("delete abc");
        assert_eq!(sut.parse(), Ok(Command::DELETE("abc".into())));
    }

    #[test]
    fn delete_cmd_no_key_test() {
        let sut: String = String::from("delete");
        assert_eq!(sut.parse::<Command>(), Err(Error::KeyNotFound)); 
    }
}
