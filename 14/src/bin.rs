mod lib;
use lib::*;

fn main() -> Result<()> {
    let formulas = parse_formulas(FOURTEEN)?;
    let fuel = find_fuel_for_ore(&formulas, 1_000_000_000_000);
    println!("fuel {}", fuel);
    assert_eq!(fuel, 11_788_286);

    Ok(())
}
