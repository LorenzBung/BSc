extern crate task2;
use task2::Config;


#[test]
fn test_parse_config_1() {
    let a = vec!["Not interested".to_string(), "e".to_string(), "Numero Due".to_string()];
    let c = Config {
        search: 'e',
        line: "Numero Due".to_string(),
    };
    assert_eq!(Config::new(&a), Ok(c));
}

#[test]
#[should_panic]
fn test_parse_config_2() {
    let a = vec!["Not interested".to_string(), "x".to_string(), "Numero Due".to_string()];
    let c = Config {
        search: 'e',
        line: "Numero Due".to_string(),
    };
    assert_eq!(Config::new(&a), Ok(c));
}

#[test]
fn test_parse_config_3() {
    let a = vec!["Not interested".to_string(), "0".to_string(), "0".to_string()];
    let c = Config {
        search: '0',
        line: "0".to_string(),
    };
    assert_eq!(Config::new(&a), Ok(c));
}

#[test]
fn test_parse_config_err_1() {
    let a = vec!["Not interested".to_string(), "e".to_string()];
    assert_eq!(Config::new(&a), Err("not enough arguments"));
}

#[test]
fn test_parse_config_err_2() {
    let a = vec!["Not interested".to_string()];
    assert_eq!(Config::new(&a), Err("not enough arguments"));
}

#[test]
fn test_run_1() {
    let c = Config {
        search: 'e',
        line: "Numero Due".to_string(),
    };
    assert_eq!(task2::run(&c), 2);
}

#[test]
fn test_run_2() {
    let c = Config {
        search: '♥',
        line: "♥ The quick brown fox jumps over the lazy dog. ♥".to_string(),
    };
    assert_eq!(task2::run(&c), 2);
}

#[test]
#[should_panic]
fn test_run_3() {
    let c = Config {
        search: 'q',
        line: "♥ The quick brown fox jumps over the lazy dog. ♥".to_string(),
    };
    assert_eq!(task2::run(&c), 2);
}

#[test]
fn test_run_4() {
    let c = Config {
        search: '!',
        line: "♥ The quick brown fox jumps over the lazy dog. ♥".to_string(),
    };
    assert_eq!(task2::run(&c), 0);
}
