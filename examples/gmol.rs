use balsa::{Atom, Element};

fn gmol(mol: &[Atom]) -> Result<f64, ()> {
    let mut gmol = 0.;
    for atom in mol {
        gmol += match atom.element().ok_or(())? {
            Element::C => 12.01,
            Element::N => 14.007,
            Element::O => 15.999,
            Element::H => 1.008,
            _ => todo!(),
        };
        gmol += atom.hydrogens() as f64 * 1.008;
    }
    Ok(gmol)
}

fn main() {
    let co2 = balsa::read("O=C=O").unwrap();
    assert_eq!(balsa::write(&co2), "O=C=O");

    let ch4 = balsa::read("C").unwrap();
    assert_eq!(balsa::write(&ch4), "C");

    println!("CO2 g/mol {:.3}", gmol(&co2).unwrap());
    println!("CH4 g/mol {:.3}", gmol(&ch4).unwrap());
}
