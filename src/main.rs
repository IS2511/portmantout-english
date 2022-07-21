use cast::u8;
use rand::Rng;
use std::{
    borrow::Cow,
    convert::Infallible,
    error::Error,
    fmt,
    fs::File,
    io::{self, BufRead, BufReader, Read},
};
use tap::Pipe;

#[derive(Debug, Clone)]
struct PmtWord<'a> {
    word: Cow<'a, str>,
    end_at: u8,
}

impl<'a> PmtWord<'a> {
    pub fn new(word: impl Into<Cow<'a, str>>, cut: u8) -> Result<Self, &'static str> {
        let word = word.into();
        // I recommend create error type for this
        if (cut as usize) > word.len() {
            Err("Cut point too far, word too short")
        } else if cut == 0 {
            Err("Cut cannot be zero")
        } else {
            Ok(PmtWord { word, end_at: cut })
        }
    }
    // pub fn word(&self) -> String {
    //     self.word.clone()
    // }
    pub fn cut(&self) -> Option<&str> {
        self.word.get(..(self.end_at as usize))
    }
}

impl fmt::Display for PmtWord<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.word[..(self.end_at as usize)])
    }
}

// struct PmtFull<'a> {
//     words: Vec<PmtWord<'a>>,
// }
//
// impl PmtFull {
//     pub fn new() -> Self {
//         PmtFull { words: Vec::new() }
//     }

// pub fn push(&mut self, elem: PmtWord) {
//
// }
// }
// type PmtFull<'a> = Vec<PmtWord<'a>>;

#[derive(Debug, Clone)]
struct PmtTreeWord<'a> {
    prev_word_cut: u8,
    // previous -> current
    // Possible use `Cow<'a, str>`
    word: &'a str,
    word_end: Vec<PmtTreeWord<'a>>,
}

// Impl Iterator for `PmtTreeWord` or add `.iter()`/`.into_iter()` methods
impl Iterator for PmtTreeWord<'_> {
    // later use never type !
    type Item = Infallible;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("please implement `Iterator`")
    }
}

// 01234567   0123456
// previous + current => ouscurrent
//      ^     ^^^^^^^ <- word
// |----| <- prev_word_cut
// ^^^^^^^^ <- word_end[123]

impl<'a> PmtTreeWord<'a> {
    pub fn new_root(word: &'a str) -> Self {
        PmtTreeWord {
            prev_word_cut: 0,
            word,
            word_end: Vec::new(),
        }
    }

    // so strange logic
    pub fn push(&self, _: &str, prev_word_cut: u8) -> Result<&PmtTreeWord, &'static str> {
        if (prev_word_cut as usize) >= self.word.len() {
            return Err("cut point too far, previous word too short");
        } else {
            Ok(self)
        }
    }

    // Find and return the PmtTreeWord that contains the given word, traversing the tree
    pub fn find_word(&self, word: &str) -> Option<&PmtTreeWord> {
        if self.word != word {
            self.word_end.iter().find_map(|sub| sub.find_word(word))
        } else {
            Some(self)
        }
    }
}

impl<'a> FromIterator<PmtWord<'a>> for PmtTreeWord<'_> {
    fn from_iter<I: IntoIterator<Item = PmtWord<'a>>>(_: I) -> Self {
        todo!("please impl me")
    }
}

fn build_pmt_from_iterator(
    words: impl Iterator<Item = io::Result<String>>,
) -> io::Result<Vec<PmtWord<'static>>> {
    // is nightly feature + my ext for infer type in `stable_try`
    fn other_box(error: Box<dyn Error + Send + Sync>) -> io::Error {
        io::Error::new(io::ErrorKind::Other, error)
    }

    // stable try block
    macro_rules! stable_try {
        ($($t:tt)*) => {
            (|| {
                $($t)*
            })()
        };
    }

    // Unlike your implementation it does not skip elements but returns an error
    // if you want old behavior, use `.flat_map`
    words
        .map(|word| {
            word.and_then(|word| {
                stable_try! {
                    u8(word.len())?
                        .pipe(|len| PmtWord::new(word, len))?
                        .pipe(Ok)
                }
                .map_err(other_box)
            })
        })
        .collect()
}

// Not use owned vector to where it is not necessary to transfer ownership
// also you can use `Cow<'a, str>` or `&str`
// and rename function
fn build_pmt_from_lifetime<'a>(words: &[String]) -> PmtTreeWord<'a> {
    let mut pmt: PmtTreeWord =
        PmtTreeWord::new_root(&words[rand::thread_rng().gen_range(0, words.len())]);

    //for word in words {
    //    let pmt_word = PmtTreeWord::new();
    //}

    // Instead of
    // for word in words {
    //     let pmt_word = PmtWord::new(word.clone(), word.len() as u8);
    //     if pmt_word.is_ok() {
    //         pmt.push(pmt_word.unwrap());
    //     }
    // }
    // use iterators with skip `Err`
    words
        .iter()
        .flat_map(|word| u8(word.len()).map(|len| PmtWord::new(word, len)))
        // fixme: i use two `flat_map` to ignore `Result<Result<PmtWord, Err1>, Err2>`
        //  create custom `Error` type which has { usize to u8 } overflow case
        //  and use `.flatten()`
        .flat_map(|x| x)
        // fixme: I impl empty `FromIterator` - add logic
        .collect()
    // or with return `Err` into function (change signature)
    // Result<PmtTreeWord, _> = words
    //     .iter()
    //     .map(|word| {
    //         u8(word.len())
    //             // fixme: also create custom `Error` type
    //             .map_err(|_| "u8 overflow")
    //             .map(|len| PmtWord::new(word, len))
    //     })
    //     .collect()
}

fn print_help(this_name: &str) {
    println!("Usage: {this_name} <dict-file> [-h|--help]");
}

// fs can return error, don't ignore it
fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args().collect();

    // Use the clap crate. Torment will not help you learn Rust
    for arg in args.iter() {
        if (arg == "--help") || (arg == "-h") {
            print_help(&args[0]);
            todo!("handle arg parsing")
        }
    }
    if args.len() < 2 {
        print_help(&args[0]);
        todo!("handle arg parsing")
    }

    let dict_filename = &args[1];
    // now instead of
    let dict_file = File::open(dict_filename).expect("Couldn't open file.");
    // you can write
    // let dict_file = File::open(dict_filename)?;
    let reader = BufReader::new(dict_file);

    // let words: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    // your:
    let mut words: Vec<String> = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            words.push(line);
        }
    }
    // suggest:
    // let words: io::Result<Vec<_>> = reader.lines().collect();

    let pmt = build_pmt_from_lifetime(&words);

    for word in pmt {
        print!("{word} ");
    }
    println!();

    Ok(())
}
