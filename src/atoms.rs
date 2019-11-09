fn get_element_label_from_element_id(id: &usize) -> &'static str {
    match id {
        1 => "H",
        2 => "He",
        3 => "Li",
        4 => "Be",
        5 => "B",
        6 => "C",
        7 => "N",
        8 => "O",
        9 => "F",
        10 => "Ne",
        11 => "Na",
        12 => "Mg",
        13 => "Al",
        14 => "Si",
        15 => "P",
        16 => "S",
        17 => "Cl",
        _ => "??",
    }
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
        let (element, _charge) = Self::decode(encoding);
        get_bonds_for_element(element as usize)
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
