use national_grid;

fn main() {
    let coordinates = national_grid::reference_to_coordinates(&"HY");
    println!("{}", coordinates)
}
