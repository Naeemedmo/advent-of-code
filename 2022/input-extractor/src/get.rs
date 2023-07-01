use curl::easy::{Easy2, Handler, WriteError};

pub fn get_input(input_url: &str, session_cookie: &str)  -> String {

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