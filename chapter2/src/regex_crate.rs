use regex::Regex;

pub fn match_regex(re: Regex) {
    let quote = "\
     Every face, every shop, bedroom window, public-house, and
     dark square is a picture feverishly turned--in search of what?
     It is the same with books.
     What do we seek through millions of pages?";

    for line in quote.lines() {
        match re.find(line) {
            Some(_) => {
                println!("--- regex match for {}: ---", re.as_str());
                println!("{}", line.trim());
            },
            None => continue,
        }
    }
}
