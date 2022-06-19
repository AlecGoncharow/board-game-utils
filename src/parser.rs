use super::schema::BoardGames;
use anyhow::Result;
use std::fs::read_to_string;
use std::path::Path;

pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<BoardGames> {
    let data = read_to_string(path)?;

    Ok(ron::de::from_str(&data)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    static PROJECT_PATH: &'static str = env!("CARGO_MANIFEST_DIR");
    static TESTS_PATH: &'static str = "/examples/";

    fn make_path(s: &str) -> String {
        PROJECT_PATH.to_owned() + TESTS_PATH + s + ".ron"
    }

    fn load_str(s: &str) -> Result<BoardGames> {
        let path = make_path(s);
        Ok(load_from_path(path)?)
    }

    macro_rules! test_ok {
        ($name:ident) => {
            #[test]
            fn $name() {
                let load = load_str(&stringify!($name));
                dbg!(&load);
                assert!(load.is_ok());
            }
        };
    }

    #[allow(unused_macros)]
    macro_rules! test_err {
        ($name:ident) => {
            #[test]
            fn $name() {
                let load = load_str(&stringify!($name));
                dbg!(&load);
                assert!(load.is_err());
            }
        };
    }

    test_ok!(simple);
    test_ok!(minimal);
}
