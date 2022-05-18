use std::fmt;
use std::io::BufRead;

#[derive(Debug, Clone)]
struct PmtWord {
    word: String,
    cut: u8,
}

impl PmtWord {
    pub fn new(word: String, cut: u8) -> Result<Self, &'static str> {
        if (cut as usize) > word.len() {
            return Err("Cut point too far, word too short")
        }
        if cut == 0 {
            return Err("Cut cannot be zero");
        }
        Ok(PmtWord { word, cut })
    }
    // pub fn word(&self) -> String {
    //     self.word.clone()
    // }
    pub fn cut(&self) -> Option<String> {
        let word_cut = self.word.get(..(self.cut as usize));
        if word_cut.is_some() {
            return Some(String::from(word_cut.unwrap()));
        }
        None
    }
}

impl fmt::Display for PmtWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.word.get(..(self.cut as usize))
            .expect("Cut point too far, word too short"))
    }
}

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

fn build_pmt(mut words: impl Iterator<Item = std::io::Result<String>>) -> Vec<PmtWord> {
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

    let pmt = build_pmt(reader.lines());

    for word in pmt {
        print!("{} ", word);
    }
    println!();
}
