use crate::bng_error::{BngError, BngResult};
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
pub struct ReferenceString(String);

impl ReferenceString {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl FromStr for ReferenceString {
    type Err = BngError;

    fn from_str(s: &str) -> BngResult<ReferenceString> {
        let upper = s.to_uppercase();
        let re = Regex::new(r"^[HJNOSW][^I](\d\d){0,5}$").unwrap();
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
