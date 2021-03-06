use crate::bng_error::{BngError, BngResult};
use crate::constants::{GRID, GRIDSIZE};
use crate::coordinate::{BngCoordinates, Eastings, Northings};
use num::Integer;
use regex::Regex;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[derive(Debug)]
pub struct ReferenceString(String);

impl Deref for ReferenceString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for ReferenceString {
    type Err = BngError;

    fn from_str(s: &str) -> BngResult<ReferenceString> {
        let upper = s.to_uppercase();
        let re = Regex::new(r"^[HJNOST][^I](\d\d){0,5}$").unwrap();
        if re.is_match(&upper) {
            Ok(ReferenceString { 0: upper })
        } else {
            match upper {
                upper if upper.len() == 1 => {
                    Err(BngError::InvalidReferenceString("Too short".to_string()))
                }
                upper if upper.len() > 12 => {
                    Err(BngError::InvalidReferenceString("Too long".to_string()))
                }
                upper if upper.len() % 2 != 0 => {
                    Err(BngError::InvalidReferenceString("Odd".to_string()))
                }
                _ => Err(BngError::InvalidReferenceString(
                    "Something else".to_string(),
                )),
            }
        }
    }
}

#[derive(Debug)]
pub struct BngLetters(String);

impl Deref for BngLetters {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for BngLetters {
    type Err = BngError;

    fn from_str(s: &str) -> BngResult<BngLetters> {
        let upper = s.to_uppercase();
        let re = Regex::new(r"^[HJNOST][^I]$").unwrap();
        if re.is_match(&upper) {
            Ok(BngLetters { 0: upper })
        } else {
            match upper {
                upper if upper.len() == 1 => Err(BngError::InvalidLetters("Too short".to_string())),
                upper if upper.len() > 2 => Err(BngError::InvalidLetters("Too long".to_string())),
                _ => Err(BngError::InvalidLetters("Something else".to_string())),
            }
        }
    }
}

#[derive(Debug)]
pub struct BngCoordinateRemainder(String);

impl BngCoordinateRemainder {
    pub fn reduce_resolution(
        self,
        from_resolution: usize,
        to_resolution: usize,
    ) -> BngResult<BngCoordinateRemainder> {
        let coordinate_remainder_string = self.deref().as_str();
        let coordinate_remainder_length = coordinate_remainder_string.len();

        let from_resolution = from_resolution as f32;
        let from_resolution_digits = from_resolution.log10();

        let to_resolution = to_resolution as f32;
        let to_resolution_digits = to_resolution.log10();

        let difference = to_resolution_digits - from_resolution_digits;
        let difference = difference as usize;

        let mid_point = coordinate_remainder_length - difference;

        let split = coordinate_remainder_string.split_at(mid_point);

        return BngCoordinateRemainder::from_str(split.0);
    }
}

impl Deref for BngCoordinateRemainder {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BngCoordinateRemainder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for BngCoordinateRemainder {
    type Err = BngError;

    fn from_str(s: &str) -> BngResult<BngCoordinateRemainder> {
        let upper = s.to_uppercase();
        let re = Regex::new(r"^(\d){0,5}$").unwrap();
        if re.is_match(&upper) {
            Ok(BngCoordinateRemainder { 0: upper })
        } else {
            match upper {
                upper if upper.len() > 5 => {
                    Err(BngError::InvalidCoordinateRemainder("Too long".to_string()))
                }
                upper if upper.chars().any(|character| !character.is_numeric()) => {
                    Err(BngError::InvalidCoordinateRemainder(
                        "Contains non-numeric character".to_string(),
                    ))
                }
                _ => Err(BngError::InvalidCoordinateRemainder(
                    "Something else".to_string(),
                )),
            }
        }
    }
}

#[derive(Debug)]
pub struct Reference {
    pub letters: BngLetters,
    pub eastings: Option<BngCoordinateRemainder>,
    pub northings: Option<BngCoordinateRemainder>,
    pub resolution: usize,
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
    pub fn reduce_resolution(self, to_resolution: usize) -> BngResult<Reference> {
        let from_resolution = self.resolution;
        if to_resolution < from_resolution {
            Err(BngError::Other("Cannot increase resolution.".to_string()))
        } else if to_resolution == from_resolution {
            Ok(self)
        } else {
            let eastings = self
                .eastings
                .unwrap()
                .reduce_resolution(from_resolution, to_resolution)?;
            let northings = self
                .northings
                .unwrap()
                .reduce_resolution(from_resolution, to_resolution)?;
            Ok(Reference {
                letters: self.letters,
                eastings: Some(eastings),
                northings: Some(northings),
                resolution: to_resolution,
            })
        }
    }
}

impl FromStr for Reference {
    type Err = BngError;

    fn from_str(s: &str) -> BngResult<Reference> {
        let reference_string = ReferenceString::from_str(s)?;
        let resolution: usize = get_resolution(reference_string.as_str());
        if resolution == 100_000 {
            Ok(Reference {
                letters: BngLetters::from_str(reference_string.as_str()).unwrap(),
                eastings: None,
                northings: None,
                resolution,
            })
        } else {
            let (letters, numbers): (&str, &str) = reference_string.split_at(2);
            let midpoint: usize = numbers.len() / 2;
            let (eastings, northings): (&str, &str) = numbers.split_at(midpoint);
            Ok(Reference {
                letters: BngLetters::from_str(letters).unwrap(),
                eastings: Some(BngCoordinateRemainder::from_str(eastings).unwrap()),
                northings: Some(BngCoordinateRemainder::from_str(northings).unwrap()),
                resolution,
            })
        }
    }
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut eastings = "";
        if let Some(potential_eastings) = self.eastings.as_ref() {
            eastings = potential_eastings.as_str();
        };
        let mut northings = "";
        if let Some(potential_northings) = self.northings.as_ref() {
            northings = potential_northings.as_str();
        };
        write!(f, "{}{}{}", self.letters.as_str(), eastings, northings,)
    }
}

#[derive(Debug)]
pub struct BngParts {
    pub quotient: usize,
    pub remainder: BngCoordinateRemainder,
}

impl BngParts {
    fn new(quotient: usize, remainder: BngCoordinateRemainder) -> Self {
        BngParts {
            quotient: quotient,
            remainder: remainder,
        }
    }
}

macro_rules! implement_try_from_for_bng_parts {
    ($($identity:ident),*) => {$(
        impl TryFrom<$identity> for BngParts {
            type Error = BngError;
            fn try_from(coordinate: $identity) -> BngResult<BngParts> {
                let (coordinate_quotient, coordinate_remainder) = coordinate.div_rem(&GRIDSIZE);
                let coordinate_remainder_string = format!("{:0>5}", coordinate_remainder);
                let coordinate_remainder = BngCoordinateRemainder::from_str(&coordinate_remainder_string)?;
                Ok(BngParts::new(coordinate_quotient, coordinate_remainder))
            }
        }
    )*}
}

implement_try_from_for_bng_parts!(Eastings, Northings);

#[derive(Debug)]
pub struct BngCoordinateParts {
    pub eastings: BngParts,
    pub northings: BngParts,
}

impl BngCoordinateParts {
    fn new(eastings: BngParts, northings: BngParts) -> Self {
        BngCoordinateParts {
            eastings: eastings,
            northings: northings,
        }
    }
}

impl TryFrom<BngCoordinates> for BngCoordinateParts {
    type Error = BngError;
    fn try_from(coordinate: BngCoordinates) -> BngResult<Self> {
        let eastings_parts = BngParts::try_from(coordinate.eastings)?;
        let northings_parts = BngParts::try_from(coordinate.northings)?;
        Ok(BngCoordinateParts::new(eastings_parts, northings_parts))
    }
}

impl TryFrom<BngCoordinateParts> for Reference {
    type Error = BngError;
    fn try_from(coordinate_parts: BngCoordinateParts) -> BngResult<Self> {
        let letter_string =
            GRID[coordinate_parts.northings.quotient][coordinate_parts.eastings.quotient];
        let letters = BngLetters::from_str(letter_string).unwrap();
        Ok(Reference {
            letters: letters,
            eastings: Some(coordinate_parts.eastings.remainder),
            northings: Some(coordinate_parts.northings.remainder),
            resolution: 1,
        })
    }
}

impl TryFrom<BngCoordinates> for Reference {
    type Error = BngError;
    fn try_from(coordinate: BngCoordinates) -> BngResult<Self> {
        let parts = BngCoordinateParts::try_from(coordinate)?;
        let reference = Reference::try_from(parts)?;
        Ok(reference)
    }
}
