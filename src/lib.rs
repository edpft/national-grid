pub mod error;
pub mod reference_string;

use error::{BngError, BngResult};
use geo_types::{CoordNum, Coordinate};
use num::cast::cast;
use num::Integer;
use reference_string::ReferenceString;
use std::convert::From;
use std::fmt;

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
        _ => 1, //TODO error?
    }
}

impl From<ReferenceString> for Reference {
    fn from(reference_string: ReferenceString) -> Self {
        let string = reference_string.value();
        let resolution: usize = get_resolution(string);
        let (letters, numbers): (&str, &str) = string.split_at(2);
        let midpoint: usize = numbers.len() / 2;
        let (eastings, northings): (&str, &str) = numbers.split_at(midpoint);
        Reference {
            letters: letters.to_string(),
            eastings: Some(eastings.to_string()),
            northings: Some(northings.to_string()),
            resolution,
        }
    }
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.letters,
            self.eastings.as_deref().unwrap_or("[Unknown]"),
            self.northings.as_deref().unwrap_or("[Unknown]")
        )
    }
}

pub trait ToBngParts {
    fn to_bng_parts(&self) -> BngResult<(usize, String)>;
}

impl<T> ToBngParts for T
where
    T: CoordNum,
{
    fn to_bng_parts(&self) -> BngResult<(usize, String)> {
        let coordinate: usize = cast(*self).unwrap();
        let (coordinate_quotient, coordinate_remainder) = coordinate.div_rem(&100_000usize);
        let coordinate_remainder = format!("{:0>5}", coordinate_remainder);
        Ok((coordinate_quotient, coordinate_remainder))
    }
}

impl<T> TryFrom<Coordinate<T>> for Reference
where
    T: CoordNum,
{
    type Error = BngError;

    fn try_from(coordinate: Coordinate<T>) -> Result<Self, Self::Error> {
        let (eastings_quotient, eastings_remainder) = coordinate.x.to_bng_parts()?;
        let (northings_quotient, northings_remainder) = coordinate.y.to_bng_parts()?;
        Ok(Reference {
            letters: GRID[northings_quotient][eastings_quotient].to_string(),
            eastings: Some(eastings_remainder),
            northings: Some(northings_remainder),
            resolution: 1,
        })
    }
}
