use std::env;
mod get;
use crate::get::get_input;

pub fn build_url(year: &str, day: &str) -> String {
    return format!("https://adventofcode.com/{}/day/{}/input", year, day);
}

pub fn build_cookie(cookie: &str) -> String {
    return format!("session={}", cookie);
}

// Print a web page onto stdout
fn main() {

    let args: Vec<String> = env::args().collect();
    let year: &str = &args[1]; 
    let day: &str = &args[2]; 

    // build url
    let input_url = build_url(&year, &day);
    
    // Get the session cookie from environment
    let cookie = env::var("AOC_SESSION_COOKIE").expect("Error: AOC_SESSION_COOKIE must be set!");
    let session_cookie = build_cookie(&cookie);

    let content = get_input(&input_url, &session_cookie);
    println!("{}", content);
}