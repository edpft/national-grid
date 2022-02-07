use national_grid::reference_string::ReferenceString;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let reference_string = ReferenceString::from_str("AB01234567891")?;
    println!("{:?}", reference_string);
    Ok(())
}
