use std::fmt;
use std::io::{BufRead, Read};
use rand::Rng;

// #[derive(Debug, Clone)]
// struct PmtWord {
//     word: String,
//     end_at: u8,
// }
//
// impl PmtWord {
//     pub fn new(word: String, cut: u8) -> Result<Self, &'static str> {
//         if (cut as usize) > word.len() {
//             return Err("Cut point too far, word too short")
//         }
//         if cut == 0 {
//             return Err("Cut cannot be zero");
//         }
//         Ok(PmtWord { word, end_at: cut })
//     }
//     // pub fn word(&self) -> String {
//     //     self.word.clone()
//     // }
//     pub fn cut(&self) -> Option<String> {
//         let word_cut = self.word.get(..(self.end_at as usize));
//         if word_cut.is_some() {
//             return Some(String::from(word_cut.unwrap()));
//         }
//         None
//     }
// }
//
// impl fmt::Display for PmtWord {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.word.get(..(self.end_at as usize))
//             .expect("Cut point too far, word too short"))
//     }
// }

// struct PmtFull<'a> {
//     words: Vec<PmtWord<'a>>,
// }

// impl PmtFull {
//     pub fn new() -> Self {
//         PmtFull { words: Vec::new() }
//     }
//
//     // pub fn push(&mut self, elem: PmtWord) {
//     //
//     // }
// }
// type PmtFull<'a> = Vec<PmtWord<'a>>;

#[derive(Debug, Clone)]
struct PmtTreeWord<'a> {
    prev_word_cut: u8, // previous -> current
    word: &'a String,
    word_end: Vec<PmtTreeWord<'a>>
}

// 01234567   0123456
// previous + current => ouscurrent
//      ^     ^^^^^^^ <- word
// |----| <- prev_word_cut
// ^^^^^^^^ <- word_end[123]

impl PmtTreeWord {
    pub fn new_root(word: &String) -> Self {
        PmtTreeWord {
            prev_word_cut: 0,
            word,
            word_end: Vec::new()
        }
    }

    pub fn push(&self, word: &String, prev_word_cut: u8) -> Result<&Self, &'static str> {
        if (prev_word_cut as usize) >= self.word.len() {
            return Err("cut point too far, previous word too short")
        }
        Ok(self)
    }

    // Find and return the PmtTreeWord that contains the given word, traversing the tree
    pub fn find_word(&self, word: &String) -> Option<&Self> {
        if self.word == word {
            return Some(self)
        }
        for sub_word in &self.word_end {
            let sub_find = sub_word.find_word(word);
            if sub_find.is_some() {
                return sub_find;
            }
        }
        None
    }
}

fn build_pmt_from_iterator(mut words: impl Iterator<Item = std::io::Result<String>>) -> Vec<PmtWord> {
    let mut pmt: Vec<PmtWord> = Vec::new();

    // let worker = || {
    //
    // }
    let mut words_empty = false;
    while !words_empty {
        let word = words.next();
        if word.is_some() {
            let word_read = word.unwrap();
            if word_read.is_ok() {
                let word_ok = word_read.unwrap();
                if word_ok.len() < 256 {
                    pmt.push(PmtWord::new(word_ok.clone(), word_ok.len() as u8).unwrap());
                }
            }
        } else {
            words_empty = true;
        }
    }

    return pmt;
}

fn build_pmt_from_static(words: Vec<String>) -> PmtTreeWord {
    let mut pmt: PmtTreeWord = PmtTreeWord::new_root(&words[rand::thread_rng().gen_range(0, words.len())]);

    for word in words {
        let pmt_word = PmtTreeWord::new();
    }

    // for word in words {
    //     let pmt_word = PmtWord::new(word.clone(), word.len() as u8);
    //     if pmt_word.is_ok() {
    //         pmt.push(pmt_word.unwrap());
    //     }
    // }

    return pmt;
}

fn print_help(this_name: &str) {
    println!("Usage: {} <dict-file> [-h|--help]", this_name);
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    for arg in args.iter() {
        if (arg == "--help") || (arg == "-h") {
            print_help(&args[0]);
            return;
        }
    }
    if args.len() < 2 {
        print_help(&args[0]);
        return;
    }

    let dict_filename: &str = &args[1];
    let dict_file = std::fs::File::open(dict_filename)
        .expect("Couldn't open file.");
    let reader = std::io::BufReader::new(dict_file);

    // let words: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let mut words: Vec<String> = Vec::new();
    for line in reader.lines() {
        if line.is_ok() {
            words.push(line.unwrap());
        }
    }
    let pmt = build_pmt_from_static(words);

    for word in pmt {
        print!("{} ", word);
    }
    println!();
}
