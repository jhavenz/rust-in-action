use std::fmt::Display;
use regex::Regex;

pub enum SearchTerm {
    Single(String),
    Multiple(Vec<String>),
    Regex(String),
    MultipleRegex(Vec<Regex>),
}

impl Display for SearchTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SearchTerm::Single(term) => write!(f, "{}", term),
            SearchTerm::Multiple(terms) => {
                let terms_as_string = terms.join(", ");
                write!(f, "{}", terms_as_string)
            }
            SearchTerm::Regex(term) => write!(f, "{}", term),
            SearchTerm::MultipleRegex(terms) => {
                let terms_as_string: Vec<String> = terms.iter().map(|term| term.to_string()).collect();
                let terms_as_string = terms_as_string.join(", ");
                write!(f, "{}", terms_as_string)
            }
        }
    }
}

#[allow(dead_code)]
impl SearchTerm {
    pub fn from_string(search_term: String) -> Self {
        if search_term.contains(",") {
            let terms: Vec<String> = search_term.split(",").map(|term| term.trim().to_string()).collect();
            SearchTerm::Multiple(terms)
        } else {
            SearchTerm::Single(search_term)
        }
    }

    pub fn from_regex(search_term: String) -> Self {
        if search_term.contains(",") {
            let terms: Vec<Regex> = search_term.split(",").map(|term| Regex::new(term.trim()).unwrap()).collect();
            SearchTerm::MultipleRegex(terms)
        } else {
            SearchTerm::Regex(search_term)
        }
    }

    pub fn matches(&self, line: &str) -> bool {
        match self {
            SearchTerm::Single(term) => line.contains(term),
            SearchTerm::Multiple(terms) => {
                for term in terms {
                    if line.contains(term) {
                        return true;
                    }
                }
                false
            }
            SearchTerm::Regex(term) => {
                let re = Regex::new(term).unwrap();
                re.is_match(line)
            }
            SearchTerm::MultipleRegex(terms) => {
                for term in terms {
                    if term.is_match(line) {
                        return true;
                    }
                }
                false
            }
        }
    }
}

pub fn grep(search_term: SearchTerm, num_lines: usize) {
    let quote = "\
     Every face, every shop, bedroom window, public-house, and
     dark square is a picture feverishly turned--in search of what?
     It is the same with books.
     What do we seek through millions of pages?";

    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];
    let bounding_ctx = num_lines.saturating_sub(1).min(1);

    for (i, line) in quote.lines().enumerate() {
        if search_term.matches(line) {
            tags.push(i);

            let v = Vec::with_capacity(bounding_ctx);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in quote.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(bounding_ctx);
            let upper_bound = tag + bounding_ctx;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line.trim());
                let local_ctx = (i, line_as_string);

                ctx[j].push(local_ctx);
            }
        }
    }


    println!("--- matches for '{}' ---", search_term);

    for num_line in ctx.iter() {
        for &(i, ref line) in num_line.iter() {
            let line_num = i + 1;

            println!("{}: {}", line_num, line);
        }
    }
}
