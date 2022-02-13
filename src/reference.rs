// use crate::bng_error::{BngError, BngResult};
use crate::reference_string::ReferenceString;
// use geo_types::{CoordNum, Coordinate};
// use num::{cast, Integer};
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

const GRIDSIZE: usize = 100_000;

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

impl Reference {
    pub fn to_coordinates(&self) -> (usize, usize) {
        let northings_quotient = GRID
            .map(|element| element.contains(&&*self.letters))
            .iter()
            .position(|element| element == &true)
            .unwrap();
        let eastings_quotient = GRID[northings_quotient]
            .iter()
            .position(|element| element == &&self.letters)
            .unwrap();
        if self.northings.is_some() & self.eastings.is_some() {
            let northings_string = self.northings.as_deref().unwrap();
            let northings_remainder = northings_string.parse::<usize>().unwrap();
            let northings = northings_quotient * GRIDSIZE + northings_remainder;

            let eastings_string = self.eastings.as_deref().unwrap();
            let eastings_remainder = eastings_string.parse::<usize>().unwrap();
            let eastings = eastings_quotient * GRIDSIZE + eastings_remainder;

            (eastings, northings)
        } else {
            (eastings_quotient * GRIDSIZE, northings_quotient * GRIDSIZE)
        }
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
            self.eastings.as_deref().unwrap(),
            self.northings.as_deref().unwrap()
        )
    }
}

// pub trait ToBngParts {
//     fn to_bng_parts(&self) -> BngResult<(usize, String)>;
// }

// impl<T> ToBngParts for T
// where
//     T: CoordNum,
// {
//     fn to_bng_parts(&self) -> BngResult<(usize, String)> {
//         let coordinate: usize = cast::cast(*self).unwrap();
//         let (coordinate_quotient, coordinate_remainder) = coordinate.div_rem(&100_000usize);
//         let coordinate_remainder = format!("{:0>5}", coordinate_remainder);
//         Ok((coordinate_quotient, coordinate_remainder))
//     }
// }

// impl<T> TryFrom<Coordinate<T>> for Reference
// where
//     T: CoordNum,
// {
//     type Error = BngError;

//     fn try_from(coordinate: Coordinate<T>) -> Result<Self, Self::Error> {
//         let (eastings_quotient, eastings_remainder) = coordinate.x.to_bng_parts()?;
//         let (northings_quotient, northings_remainder) = coordinate.y.to_bng_parts()?;
//         Ok(Reference {
//             letters: GRID[northings_quotient][eastings_quotient].to_string(),
//             eastings: Some(eastings_remainder),
//             northings: Some(northings_remainder),
//             resolution: 1,
//         })
//     }
// }
