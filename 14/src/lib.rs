use std::collections::HashMap;
use std::convert::TryFrom;

pub type Equations<'a> = HashMap<&'a str, Equation<'a>>;
pub type ChemAmount = usize;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Chem<'a> {
    amount: ChemAmount,
    name: &'a str,
}

impl<'a> TryFrom<&'a str> for Chem<'a> {
    type Error = ();

    fn try_from(input: &str) -> std::result::Result<Chem, Self::Error> {
        let components: Vec<&str> = input.trim().split(' ').collect();
        let amount: ChemAmount = match components[0].parse() {
            Ok(x) => x,
            Err(_) => panic!("unparsable quantity"),
        };
        let name = components[1];

        Ok(Chem { amount, name })
    }
}

#[derive(Debug)]
pub struct Equation<'a> {
    inputs: Vec<Chem<'a>>,
    output: Chem<'a>,
}

pub fn parse_formulas<'a>(input: &'a str) -> Result<Equations> {
    let mut formula = Equations::new();
    for line in input.lines() {
        let eq: Vec<&str> = line.trim().split("=>").collect();
        let lhs = eq[0].trim();
        let rhs = eq[1].trim();

        let inputs: Vec<Chem<'a>> = lhs.split(',').map(|s| Chem::try_from(s).unwrap()).collect();
        let output = Chem::try_from(rhs).unwrap();
        let equasion = Equation { inputs, output };

        formula.insert(output.name, equasion);
    }

    Ok(formula)
}

pub fn ore_for_fuel(equations: &Equations, fuel: ChemAmount) -> ChemAmount {
    let mut ore = 0;
    let mut to_make: Vec<Chem> = Vec::new();
    let mut extra: HashMap<&str, usize> = equations.keys().map(|&k| (k, 0)).collect();

    to_make.push(Chem {
        name: "FUEL",
        amount: fuel,
    });

    while let Some(chem_to_make) = to_make.pop() {
        if chem_to_make.name == "ORE" {
            ore += chem_to_make.amount;
            continue;
        }

        // Check slush fund and use that first
        let needed = if chem_to_make.amount <= extra[chem_to_make.name] {
            *extra.get_mut(chem_to_make.name).unwrap() -= chem_to_make.amount;
            0
        } else {
            let slush = extra[chem_to_make.name];
            chem_to_make.amount - slush
        };

        if needed > 0 {
            // After slush fund is used, use equasion to make more, queue up parts
            let equation = &equations[chem_to_make.name];

            // How many instances of chem eq need to run?
            // (x + y - 1) / y  is x/y rounded up instead of down
            let multiplier = (needed + equation.output.amount - 1) / equation.output.amount;

            // Add output to slush fund
            *extra.get_mut(equation.output.name).unwrap() += equation.output.amount * multiplier;

            // queue up inputs to make
            for &input in equation.inputs.iter() {
                to_make.push(Chem {
                    name: input.name,
                    amount: input.amount * multiplier,
                });
            }

            to_make.push(chem_to_make); // Repush since output has not been spent yet
        }
    }

    ore
}

pub fn find_fuel_for_ore(equations: &Equations, ore: ChemAmount) -> ChemAmount {
    let mut lower = 0;
    let mut upper = 8_388_608;

    let mut current_ore = 694_999_561;
    while current_ore < ore {
        lower = upper;
        upper *= 2;
        current_ore = ore_for_fuel(equations, upper);
        println!("upper, lower, ore: {}, {}, {}", upper, lower, current_ore);
    }

    while lower < upper {
        let mid = (lower + upper) / 2;
        current_ore = ore_for_fuel(equations, mid);

        println!(
            "upper, lower, mid, ore: {}, {}, {}, {}",
            upper, lower, mid, current_ore
        );
        if current_ore < ore {
            lower = mid + 1;
        } else if current_ore > ore {
            upper = mid - 1;
        } else {
            return upper;
        }
    }

    println!("upper, lower: {}, {}", upper, lower);

    lower
}

pub const FOURTEEN: &str = "1 HVXJL, 1 JHGQ => 2 ZQFQ
    6 GRQTX => 6 VZWRS
    128 ORE => 2 GRQTX
    1 MJPSW => 4 MGZBH
    3 HLQX => 8 KSMW
    4 QLNS => 9 LFRW
    10 HBCN => 3 CZWP
    1 CQRJP => 9 MJPSW
    1 SLXC => 6 SDTGP
    1 MTGVK => 4 NZWLQ
    4 PMJX => 3 CVKM
    2 LDKGL, 2 SFKF => 5 XZDV
    1 QLNS, 1 VZWRS => 5 RSBT
    1 NRQS, 22 LQFDM => 4 PMJX
    17 XZDV, 8 GSRKQ => 3 ZGDC
    11 BPJLM, 18 ZGDC, 1 JHGQ => 5 BXNJX
    2 GRQTX, 1 CQRJP => 7 NRQS
    1 LJTL => 7 DBHXK
    15 HPBQ, 5 PSPCF, 1 JHGQ, 25 ZMXWG, 1 JTZS, 1 SDTGP, 3 NLBM => 6 MQVLS
    9 KSMW => 2 GXTBV
    3 HVXJL => 5 JHGQ
    1 ZWXT, 13 MJPSW, 10 HVXJL => 5 LDKGL
    1 GRQTX => 2 LQFDM
    190 ORE => 5 FQPNW
    1 GTQB => 9 HVHN
    1 TNLN, 9 HVHN, 1 WLGT, 4 NZMZ, 2 QTPC, 1 LPTF => 7 WFCH
    3 PMJX => 5 SFKF
    1 ZGDC => 9 HTVR
    193 ORE => 1 CQRJP
    1 BPJLM, 1 HPBQ, 3 HVHN => 6 NLBM
    2 SFKF => 1 GSRKQ
    1 ZGDC => 8 GTQB
    1 LSPMR, 53 LDKGL, 24 WFCH, 32 GDLH, 2 HLQX, 14 NLBM, 18 BDZK, 7 MDSRW, 9 MQVLS => 1 FUEL
    12 SFKF => 7 NZMZ
    13 PVJM => 3 XBTH
    7 GSRKQ, 7 LPTF, 1 HLQX, 1 FJHK, 1 DHVM, 3 SFKF, 15 NLBM, 2 SDTGP => 3 LSPMR
    4 LFRW, 28 MJPSW => 4 GDLH
    6 VZWRS, 8 MJPSW => 8 HVXJL
    13 LFRW => 4 ZWQW
    1 LQFDM, 7 NZWLQ, 2 HVXJL => 4 HLQX
    2 KSMW, 1 WDGN, 4 ZQFQ => 1 ZMXWG
    3 MGZBH => 2 LPTF
    1 LFRW, 1 CVKM, 3 LDKGL => 4 LJTL
    3 LJTL, 20 CZWP, 1 HPBQ => 9 WLGT
    3 FQPNW => 8 MTGVK
    1 MTDWJ, 1 CVKM => 9 WDGN
    5 ZWQW => 3 MTDWJ
    2 CVKM => 8 QTPC
    2 PVJM, 9 ZWQW, 1 MTDWJ => 4 HBCN
    5 RSBT, 2 WDGN, 6 GSRKQ => 1 BPJLM
    34 JHGQ, 6 ZGDC => 8 DHVM
    3 QTPC, 1 RSBT, 1 GXTBV => 9 JTZS
    1 BXNJX, 2 JTZS => 5 SLXC
    23 LPTF, 2 NZMZ => 4 TNLN
    24 HTVR, 5 DBHXK => 2 FJHK
    5 LPTF, 5 QTPC => 4 PSPCF
    17 MTGVK, 27 LQFDM => 4 QLNS
    1 CVKM, 5 HTVR => 8 HPBQ
    6 ZQFQ, 28 XBTH => 7 MDSRW
    13 WDGN => 5 BDZK
    1 MJPSW, 2 VZWRS => 4 ZWXT
    1 MGZBH, 1 GRQTX => 8 PVJM";

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "10 ORE => 10 A
                            1 ORE => 1 B
                            7 A, 1 B => 1 C
                            7 A, 1 C => 1 D
                            7 A, 1 D => 1 E
                            7 A, 1 E => 1 FUEL";

    #[test]
    fn verify_parse() -> Result<()> {
        let formula = parse_formulas(EXAMPLE_1)?;
        assert_eq!(formula.len(), 6);
        assert!(formula.contains_key("A"));
        assert!(formula.contains_key("B"));
        assert!(formula.contains_key("C"));
        assert!(formula.contains_key("D"));
        assert!(formula.contains_key("E"));
        assert!(formula.contains_key("FUEL"));

        Ok(())
    }

    #[test]
    fn fourteen_1() -> Result<()> {
        let formulas = parse_formulas(FOURTEEN)?;
        let ore = ore_for_fuel(&formulas, 1);
        assert_eq!(ore, 216_477);

        Ok(())
    }

    #[test]
    fn fourteen_2() -> Result<()> {
        let formulas = parse_formulas(FOURTEEN)?;
        let fuel = find_fuel_for_ore(&formulas, 1_000_000_000_000);
        assert_eq!(fuel, 11_788_286);

        Ok(())
    }

    #[test]
    fn example_1() -> Result<()> {
        let formulas = parse_formulas(EXAMPLE_1)?;
        let ore = ore_for_fuel(&formulas, 1);
        assert_eq!(ore, 31);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        let input = "9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL";
        let formulas = parse_formulas(input)?;
        let ore = ore_for_fuel(&formulas, 1);
        assert_eq!(ore, 165);

        Ok(())
    }

    #[test]
    fn example_3() -> Result<()> {
        let input = "157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let formulas = parse_formulas(input)?;
        let ore = ore_for_fuel(&formulas, 1);
        assert_eq!(ore, 13312);

        Ok(())
    }
}
