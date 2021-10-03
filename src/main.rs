use std::string::ParseError;
use std::str::FromStr;
use ansi_term::ANSIString;
use ansi_term::Colour::Blue;
use ansi_term::Colour::White;
use ansi_term::Colour::Green;
use std::fs::DirEntry;
use structopt::StructOpt;
use std::fs;
use regex::Regex;

#[derive(Debug, PartialEq)]
enum Color {
    Auto,
    Always,
    Never,
}

impl FromStr for Color {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Color::Auto),
            "always" => Ok(Color::Always),
            "never" => Ok(Color::Never),
             _ =>  Ok(Color::Always)
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "cli", about = "A simple rust cli template.")]
struct Opt {

    #[structopt(short = "l", long = "list")]
    list: bool,

    #[structopt(short = "a", long = "all")]
    all: bool,
    
    #[structopt(short = "h", long = "hide")]
    hide: Option<String>,

    #[structopt(short = "c", long = "color", parse(try_from_str = std::str::FromStr::from_str), default_value="always")]
    color: Color,

    path: String
}

fn main() {
    let opt = Opt::from_args();
    let all = opt.all;
    let delimiter  = match opt.list {
        false => " ",
        true => "\n"
    };

   let hide_regex = opt.hide.as_ref().map_or(Regex::new("$^"), |h| Regex::new(h));
   let col = opt.color;
        
   for entry in fs::read_dir(&opt.path)
        .expect(&format!("Can't read path {}.", &opt.path))
        .map(|f| color_entry(f.unwrap(), &col))
        .filter(|n| !hide_regex.as_ref().unwrap().is_match(n))
        .filter(|n| all || !n.starts_with('.')) {
            print!("{}{}", entry, delimiter);
    }
    println!()
} 


fn color_entry(e: DirEntry, c: &Color) -> ANSIString<'static> {
    if c == &Color::Never {
      White.paint(e.file_name().into_string().unwrap())
    } else if e.file_type().unwrap().is_dir() {
        Green.paint(e.file_name().into_string().unwrap())
    } else if e.metadata().unwrap().permissions().readonly() {
        Blue.paint(e.file_name().into_string().unwrap())
    } else {
      White.paint(e.file_name().into_string().unwrap())
    }
}
