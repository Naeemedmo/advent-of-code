pub mod get {
    use crate::build::{build_cookie, build_url};
    use curl::easy::{Easy2, Handler, WriteError};
    use std::env;
    pub struct AoCDate {
        pub day: u32,
        pub year: u32,
    }
    pub fn get_input(date: AoCDate) -> String {
        let input_url = build_url(date.year, date.day);
        let cookie =
            env::var("AOC_SESSION_COOKIE").expect("Error: AOC_SESSION_COOKIE must be set!");
        let session_cookie = build_cookie(&cookie);
        struct Collector(Vec<u8>);

        impl Handler for Collector {
            fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
                self.0.extend_from_slice(data);
                Ok(data.len())
            }
        }

        let mut easy = Easy2::new(Collector(Vec::new()));
        easy.get(true).unwrap();
        easy.url(&input_url).unwrap();
        let _ = easy.cookie(&session_cookie);
        easy.perform().unwrap();

        assert_eq!(easy.response_code().unwrap(), 200);
        let contents = easy.get_ref();

        return String::from_utf8_lossy(&contents.0).to_string();
    }
}
pub mod build {
    pub fn build_url(year: u32, day: u32) -> String {
        return format!("https://adventofcode.com/{}/day/{}/input", year, day);
    }
    pub fn build_cookie(cookie: &str) -> String {
        return format!("session={}", cookie);
    }
}
