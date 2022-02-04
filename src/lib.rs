use anyhow;
use geo_types::{CoordNum, Coordinate};
use num::cast::cast;
use num::Integer;
use std::fmt;
use std::str::FromStr;

const GRID: [[&str; 7]; 15] = [
    ["SV", "SW", "SX", "SY", "SZ", "TV", "TW"],
    ["SQ", "SR", "SS", "ST", "SU", "TQ", "TR"],
    ["SL", "SM", "SN", "SO", "SP", "TL", "TM"],
    ["SF", "SG", "SH", "SJ", "SK", "TF", "TG"],
    ["SA", "SB", "SC", "SD", "SE", "TA", "TB"],
    ["NV", "NW", "NX", "NY", "NZ", "OV", "OW"],
    ["NQ", "NR", "NS", "NT", "NU", "OQ", "OR"],
    ["NL", "NM", "NN", "NO", "NP", "OL", "OM"],
    ["NF", "NG", "NH", "NJ", "NK", "OF", "OG"],
    ["NA", "NB", "NC", "ND", "NE", "OA", "OB"],
    ["HV", "HW", "HX", "HY", "HZ", "JV", "JW"],
    ["HQ", "HR", "HS", "HT", "HU", "JQ", "JR"],
    ["HL", "HM", "HN", "HO", "HP", "JL", "JM"],
    ["HF", "HG", "HH", "HJ", "HK", "JF", "JG"],
    ["HA", "HB", "HC", "HD", "HE", "JA", "JB"],
];

#[derive(Debug)]
pub struct Reference {
    letters: String,
    eastings: Option<String>,
    northings: Option<String>,
    resolution: usize,
}

fn get_resolution(reference_string: &str) -> usize {
    match reference_string.len() {
        2 => 100_000,
        4 => 10_000,
        6 => 1_000,
        8 => 100,
        10 => 10,
        _ => 1,
    }
}

impl FromStr for Reference {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Reference> {
        let resolution: usize = get_resolution(&s);
        let (letters, numbers): (&str, &str) = s.split_at(2);
        let midpoint: usize = numbers.len() / 2;
        let (eastings, northings): (&str, &str) = numbers.split_at(midpoint);
        Ok(Reference {
            letters: letters.to_string(),
            eastings: Some(eastings.to_string()),
            northings: Some(northings.to_string()),
            resolution: resolution,
        })
    }
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.letters,
            self.eastings.as_ref().unwrap(),
            self.northings.as_ref().unwrap()
        )
    }
}

pub trait ToBngParts {
    fn to_bng_parts(&self) -> (usize, String);
}

impl<T> ToBngParts for T
where
    T: CoordNum,
{
    fn to_bng_parts(&self) -> (usize, String) {
        let coordinate: usize = cast(*self).unwrap();
        let (coordinate_quotient, coordinate_remainder) = coordinate.div_rem(&100_000usize);
        let coordinate_remainder = format!("{:0>5}", coordinate_remainder);
        (coordinate_quotient, coordinate_remainder)
    }
}

impl<T> TryFrom<Coordinate<T>> for Reference
where
    T: CoordNum,
{
    type Error = ();

    fn try_from(coordinate: Coordinate<T>) -> Result<Self, Self::Error> {
        let (eastings_quotient, eastings_remainder) = coordinate.x.to_bng_parts();
        let (northings_quotient, northings_remainder) = coordinate.y.to_bng_parts();
        Ok(Reference {
            letters: GRID[northings_quotient][eastings_quotient].to_string(),
            eastings: Some(eastings_remainder.to_string()),
            northings: Some(northings_remainder.to_string()),
            resolution: 1,
        })
    }
}
