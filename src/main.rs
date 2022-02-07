use geo_types::Coordinate;
use national_grid::Reference;

fn main() -> anyhow::Result<()> {
    let coordinate = Coordinate {
        x: 100_000,
        y: 500_000,
    };
    let reference = Reference::try_from(coordinate)?;
    println!("{:?}", reference);
    Ok(())
}
