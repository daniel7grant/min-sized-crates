use regex::bytes::Regex;
use std::str;

fn main() {
    let re = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})").unwrap();
    let hay = "On 2010-03-14, foo happened. On 2014-10-14, bar happened.".as_bytes();

    for mat in re.find_iter(hay) {
        println!(
            "Found match: {}",
            str::from_utf8(&hay[mat.start()..mat.end()]).unwrap()
        );
    }
}
