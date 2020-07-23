#[cfg(test)]
mod test {
    use command::Command;
    use command::CommandNotFoundError;
    pub use shell::Shell;
    use std::ffi::OsString;
    use std::str::FromStr;

    #[test]
    fn test_command_empty_whitespace() {
        assert_eq!(Command::from_str("       ").unwrap(), Command::Empty);
    }

    #[test]
    fn test_command_empty_tab() {
        assert_eq!(Command::from_str("\t").unwrap(), Command::Empty);
    }

    #[test]
    fn test_command_cd() {
        assert_eq!(
            Command::from_str("cd /home/peter/").unwrap(),
            Command::Cd(Some(OsString::from("/home/peter/")))
        );
    }

    #[test]
    fn test_command_home() {
        assert_eq!(Command::from_str("cd").unwrap(), Command::Cd(None));
    }

    #[test]
    fn test_command_exit() {
        assert_eq!(Command::from_str("exit").unwrap(), Command::Exit);
    }

    #[test]
    fn test_command_not_found() {
        assert_eq!(
            Command::from_str("awdeiu33we").unwrap_err(),
            CommandNotFoundError
        );
    }

    #[test]
    fn test_prompt_in_memory_with_string() {
        let input = b"echo";
        let mut output = Vec::new();

        let mut shell = Shell {
            reader: &input[..],
            writer: &mut output,
            should_exit: false,
            name: "bsys-shell".to_string(),
        };

        let output = shell.prompt().unwrap().unwrap();

        assert_eq!("echo", output);
    }

    #[test]
    fn test_loop_of_shell_which_wants_to_exit() {
        let input = b"BlaBla";
        let mut output = Vec::new();

        let mut shell = Shell {
            reader: &input[..],
            writer: &mut output,
            should_exit: true,
            name: "bsys-shell".to_string(),
        };

        let output = shell.start().unwrap();

        assert_eq!((), output);
    }

    #[test]
    fn test_loop_with_exit_command() {
        let input = b"exit";
        let mut output = Vec::new();

        let mut shell = Shell {
            reader: &input[..],
            writer: &mut output,
            should_exit: false,
            name: "bsys-shell".to_string(),
        };

        let output = shell.start().unwrap();

        assert_eq!((), output);
    }
}
