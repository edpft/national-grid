use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

const LETTERS: [&str; 156] = [
    "H", "J", "N", "O", "S", "T", "HA", "HB", "HC", "HD", "HE", "HF", "HG", "HH", "HJ", "HK", "HL",
    "HM", "HN", "HO", "HP", "HQ", "HR", "HS", "HT", "HU", "HV", "HW", "HX", "HY", "HZ", "JA", "JB",
    "JC", "JD", "JE", "JF", "JG", "JH", "JJ", "JK", "JL", "JM", "JN", "JO", "JP", "JQ", "JR", "JS",
    "JT", "JU", "JV", "JW", "JX", "JY", "JZ", "NA", "NB", "NC", "ND", "NE", "NF", "NG", "NH", "NJ",
    "NK", "NL", "NM", "NN", "NO", "NP", "NQ", "NR", "NS", "NT", "NU", "NV", "NW", "NX", "NY", "NZ",
    "OA", "OB", "OC", "OD", "OE", "OF", "OG", "OH", "OJ", "OK", "OL", "OM", "ON", "OO", "OP", "OQ",
    "OR", "OS", "OT", "OU", "OV", "OW", "OX", "OY", "OZ", "SA", "SB", "SC", "SD", "SE", "SF", "SG",
    "SH", "SJ", "SK", "SL", "SM", "SN", "SO", "SP", "SQ", "SR", "SS", "ST", "SU", "SV", "SW", "SX",
    "SY", "SZ", "TA", "TB", "TC", "TD", "TE", "TF", "TG", "TH", "TJ", "TK", "TL", "TM", "TN", "TO",
    "TP", "TQ", "TR", "TS", "TT", "TU", "TV", "TW", "TX", "TY", "TZ",
];

pub struct Reference {
    letters: String,
    eastings: String,
    northings: String,
}

impl Reference {
    fn new(letters: &str, eastings: &str, northings: &str) -> Reference {
        let numbers = 0..99_999;

        if !LETTERS.contains(&letters) {
            panic!("Letters must be in {:?}, got {}.", LETTERS, &letters);
        } else if !numbers.contains(&eastings.parse::<u32>().unwrap()) {
            panic!("Eastings must be between 0 and 99,999, got {}.", &eastings);
        } else if !numbers.contains(&northings.parse::<u32>().unwrap()) {
            panic!("Eastings must be between 0 and 99,999, got {}.", &northings);
        } else {
            Reference {
                letters: letters.to_string(),
                eastings: eastings.to_string(),
                northings: northings.to_string(),
            }
        }
    }
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.letters, self.eastings, self.northings)
    }
}

impl FromStr for Reference {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers_index: usize = s.find(char::is_numeric).unwrap();
        let (letters, numbers): (&str, &str) = s.split_at(numbers_index);
        let midpoint: usize = numbers.len() / 2;
        let (eastings, northings): (&str, &str) = numbers.split_at(midpoint);
        Ok(Reference::new(&letters, &eastings, &northings))
    }
}

fn main() {
    let reference = Reference::new("HA", "00", "00");
    let reference_string = reference.to_string();
    let reference_from_string = Reference::from_str(&reference_string).unwrap();
    println!("{}", reference_from_string);
}
