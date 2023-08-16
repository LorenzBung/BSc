#[cfg(test)]

mod tests {

    extern crate task1;

    use self::task1::parse;
    use self::task1::Command;
    use self::task1::ErrorType;
    use self::task1::ParseError;

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

    #[test]
    fn empty_with_newline_returns_correct_command() {
        assert_eq!(
            parse("\n12341234\n"),
            Err(ParseError(ErrorType::EmptyString))
        )
    }

    #[test]
    fn not_known_command_with_newline_returns_correct_command() {
        assert_eq!(
            parse("Hello\n123\n"),
            Err(ParseError(ErrorType::UnknownCommand))
        )
    }

    #[test]
    fn stage_with_newline_returns_correct_command() {
        assert_eq!(
            parse("STAGE 123\n456"),
            Ok(Command::Stage("123".to_string()))
        )
    }

    #[test]
    fn control_with_newline_returns_correct_command() {
        assert_eq!(
            parse("CONTROL 123\n456"),
            Ok(Command::Control("123".to_string()))
        )
    }

    #[test]
    fn retrieve_with_newline_returns_correct_command() {
        assert_eq!(parse("RETRIEVE 123\n456"), Ok(Command::Retrieve))
    }
}
