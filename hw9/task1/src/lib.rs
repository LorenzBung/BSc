pub fn parse(message: &str) -> Result<Command, ParseError> {
    let m: String = String::from(message.trim_right());
    let mut line = m.lines();
    match line.next() {
        Some(x) => {
            let mut str = x.split_whitespace();
            match str.next() {
                Some("STAGE") => {
                    let msg = x[6..].trim_left();
                    Ok(Command::Stage(msg.to_string()))
                }
                Some("CONTROL") => {
                    let cmd = x[8..].trim_left();
                    Ok(Command::Control(cmd.to_string()))
                }
                Some("RETRIEVE") => Ok(Command::Retrieve),
                Some(_) => Err(ParseError(ErrorType::UnknownCommand)),
                None => Err(ParseError(ErrorType::EmptyString)),
            }
        }
        None => Err(ParseError(ErrorType::EmptyString)),
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Stage(String),
    Control(String),
    Retrieve,
}

#[derive(Debug, PartialEq)]
pub struct ParseError(pub ErrorType);

#[derive(Debug, PartialEq)]
pub enum ErrorType {
    UnknownCommand,
    EmptyString,
}
