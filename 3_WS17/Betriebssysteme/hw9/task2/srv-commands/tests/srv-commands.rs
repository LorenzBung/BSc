#[cfg(test)]

mod tests {
    extern crate srv_commands;
    use self::srv_commands::*;

    #[test]
    fn empty_returns_correct_command() {
        assert_eq!(parse("\n"), Err(ParseError(ErrorType::EmptyString)))
    }

    #[test]
    fn not_known_command_returns_correct_command() {
        assert_eq!(parse("Hello\n"), Err(ParseError(ErrorType::UnknownCommand)))
    }

    #[test]
    fn stage_returns_correct_command() {
        assert_eq!(
            parse("STAGE Hello\n"),
            Ok(Command::Stage("Hello".to_string()))
        )
    }

    #[test]
    fn control_returns_correct_command() {
        assert_eq!(
            parse("CONTROL Hello\n"),
            Ok(Command::Control("Hello".to_string()))
        )
    }

    #[test]
    fn retrieve_returns_correct_command() {
        assert_eq!(parse("RETRIEVE\n"), Ok(Command::Retrieve))
    }

    #[test]
    fn retrieve_with_arguments_returns_correct_command() {
        assert_eq!(parse("RETRIEVE Hello\n"), Ok(Command::Retrieve))
    }
}
