use regex_lite::Regex;

fn main() {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let hay = "On 2010-03-14, foo happened. On 2014-10-14, bar happened.";

    for mat in re.find_iter(hay) {
        println!("Found match: {}", &hay[mat.start()..mat.end()]);
    }
}
