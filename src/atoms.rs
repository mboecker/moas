fn get_element_label_from_element_id(id: &usize) -> &'static str {
    let a = [
        "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mg", "Al", "Si", "P", "S",
        "Cl", "Ar", "K", "Ca", "Sc", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga",
        "Ge", "As", "Se", "Br", "Kr", "Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pd",
        "Ag", "Cd", "In", "Sn", "Sb", "Te", "I", "Xe", "Cs", "Ba", "La", "Ce", "Pr", "Nd", "Pm",
        "Sm", "Eu", "Gd", "Tb", "Dy", "Ho", "Er", "Tm", "Yb", "Lu", "Hf", "Ta", "W", "Re", "Os",
        "Ir", "Pt", "Au", "Hg", "Tl", "Pb", "Bi", "Po", "At", "Rn", "Fr", "Ra", "Ac", "Th", "Pa",
        "U", "Np", "Pu", "Am", "Cm", "Bk", "Cf", "Es", "Fm", "Md", "No", "Lr", "Rf", "Db", "Sg",
        "Bh", "Hs", "Mt", "Ds ", "Rg ", "Cn ", "Nh", "Fl", "Mc", "Lv", "Ts", "Og",
    ];
    a[id - 1]
}

fn get_bonds_for_element(a: usize) -> u8 {
    if a == 1 {
        1
    } else {
        (8 - (a - 2) % 8) as u8
    }
}

/// Contains metadata for the various elements.
pub struct Atoms;

impl Atoms {
    fn decode(encoding: usize) -> (u8, i8) {
        let element = (encoding & 0xFF) as u8;
        let charge = ((encoding >> 8) & 0xFF) as i8;
        (element, charge)
    }

    /// Encodes the charge and element into a number.
    pub fn encode(element: u8, charge: i8) -> usize {
        (charge as usize) << 8 | element as usize
    }

    /// Determines the amount of bonds an element can form.
    pub fn max_bonds(encoding: usize) -> u8 {
        let (element, charge) = Self::decode(encoding);
        (get_bonds_for_element(element as usize) as isize + charge as isize) as u8
    }

    /// Outputs a label given the atom labeling.
    pub fn label(encoding: usize) -> String {
        let (element, charge) = Self::decode(encoding);
        format!(
            "{}{}",
            get_element_label_from_element_id(&(element as usize)),
            if charge < 0 {
                "-"
            } else if charge > 0 {
                "+"
            } else {
                ""
            }
        )
    }
}
