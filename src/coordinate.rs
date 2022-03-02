use crate::bng_error::{BngError, BngResult};
use crate::reference::Reference;
use core::fmt;
use num::cast;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::ops::Deref;

pub const GRID: [[&str; 7]; 15] = [
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

pub const GRIDSIZE: usize = 100_000;

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
struct NonNegativeInteger(usize);

#[derive(Debug, Serialize, Deserialize)]
pub struct Eastings(NonNegativeInteger);

#[derive(Debug, Serialize, Deserialize)]
pub struct Northings(NonNegativeInteger);

#[derive(Debug, Serialize, Deserialize)]
pub struct BngCoordinates {
    pub eastings: Eastings,
    pub northings: Northings,
}

macro_rules! implement_deref_for_structs {
    ($($struct:ident),*) => {$(
        impl Deref for $struct {
            type Target = usize;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    )*};
}

macro_rules! implement_try_from_numerics_for_non_negative_integer {
    ($($type:ty),*) => {$(
        impl TryFrom<$type> for NonNegativeInteger {
            type Error = BngError;

            fn try_from(number: $type) -> BngResult<NonNegativeInteger> {
                let non_negative_integer: Option<usize> = cast::cast(number);
                if non_negative_integer.is_none() {
                    Err(BngError::NegativeNumber("BNG Coordinates cannot be negative.".to_string()))
                } else {
                    Ok(NonNegativeInteger { 0: non_negative_integer.unwrap() })
                }
            }
        }
    )*}
}

implement_deref_for_structs!(NonNegativeInteger, Eastings, Northings);

implement_try_from_numerics_for_non_negative_integer!(
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64
);

impl TryFrom<NonNegativeInteger> for Eastings {
    type Error = BngError;

    fn try_from(non_negative_integer: NonNegativeInteger) -> BngResult<Eastings> {
        if *non_negative_integer > 700_000 {
            Err(BngError::InvalidEastings(
                "Eastings must be less than 700,000.".to_string(),
            ))
        } else {
            Ok(Eastings {
                0: non_negative_integer,
            })
        }
    }
}

impl TryFrom<NonNegativeInteger> for Northings {
    type Error = BngError;

    fn try_from(non_negative_integer: NonNegativeInteger) -> BngResult<Northings> {
        if *non_negative_integer > 1_300_000 {
            Err(BngError::InvalidNorthings(
                "Northings must be less than 1,300,000".to_string(),
            ))
        } else {
            Ok(Northings {
                0: non_negative_integer,
            })
        }
    }
}

macro_rules! implement_new_from_numerics_for_bng_coordinates {
    ($($type:ty),*) => {$(
        impl TryFrom<($type, $type)> for BngCoordinates {
            type Error = BngError;

            fn try_from(tuple: ($type, $type)) -> BngResult<Self> {
                let potential_eastings = NonNegativeInteger::try_from(tuple.0)?;
                let valid_eastings = Eastings::try_from(potential_eastings)?;

                let potential_northings = NonNegativeInteger::try_from(tuple.1)?;
                let valid_northings = Northings::try_from(potential_northings)?;

                Ok(BngCoordinates {
                    eastings: valid_eastings,
                    northings: valid_northings,
                })
            }
        }
    )*}
}

implement_new_from_numerics_for_bng_coordinates!(
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64
);

impl BngCoordinates {
    pub fn new(eastings: usize, northings: usize) -> BngResult<Self> {
        let non_negative_easting = NonNegativeInteger::try_from(eastings)?;
        let valid_eastings = Eastings::try_from(non_negative_easting)?;
        let non_negative_northings = NonNegativeInteger::try_from(northings)?;
        let valid_northings = Northings::try_from(non_negative_northings)?;
        Ok(BngCoordinates {
            eastings: valid_eastings,
            northings: valid_northings,
        })
    }
}

impl fmt::Display for BngCoordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BngCoordinates {{ eastings: {}, northings: {} }}",
            *self.eastings, *self.northings
        )
    }
}

impl From<Reference> for BngCoordinates {
    fn from(reference: Reference) -> Self {
        let letters = reference.letters.as_str();
        let northings_quotient = GRID
            .map(|element| element.contains(&letters))
            .iter()
            .position(|element| element == &true)
            .unwrap();
        let eastings_quotient = GRID[northings_quotient]
            .iter()
            .position(|element| element == &letters)
            .unwrap();
        if reference.northings.is_some() & reference.eastings.is_some() {
            let northings_string = reference.northings.as_deref().unwrap();
            let northings_remainder = northings_string.parse::<usize>().unwrap();
            let northings = northings_quotient * GRIDSIZE + northings_remainder;

            let eastings_string = reference.eastings.as_deref().unwrap();
            let eastings_remainder = eastings_string.parse::<usize>().unwrap();
            let eastings = eastings_quotient * GRIDSIZE + eastings_remainder;

            BngCoordinates::new(eastings, northings).unwrap()
        } else {
            BngCoordinates::new(eastings_quotient * GRIDSIZE, northings_quotient * GRIDSIZE)
                .unwrap()
        }
    }
}
