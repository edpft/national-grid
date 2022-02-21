use crate::bng_error::{BngError, BngResult};
use core::fmt;
use num::cast;
use std::convert::TryFrom;
use std::ops::Deref;

#[derive(Debug, PartialEq, PartialOrd)]
struct NonNegativeInteger(usize);

#[derive(Debug)]
struct Eastings(NonNegativeInteger);

#[derive(Debug)]
struct Northings(NonNegativeInteger);

#[derive(Debug)]
pub struct BngCoordinates {
    eastings: Eastings,
    northings: Northings,
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
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, f32, f64
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
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, f32, f64
);

impl fmt::Display for BngCoordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BngCoordinates {{ eastings: {}, northings: {} }}",
            *self.eastings, *self.northings
        )
    }
}
