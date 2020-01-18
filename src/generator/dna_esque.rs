#![allow(dead_code)]
use crate::Graph;

pub struct DnaEsque;

impl super::MoleculeGenerator for DnaEsque {
    fn generate(size: usize) -> Graph {

        // At least two carbon atoms are needed, so two are always included.
        // Together with the first size-atom the first triangle is formed.
        // Every additional atom will add one triangle.
        // The six additional atoms are the hydrogen atoms used to fix up the missing bonds.
        let space = 2 + 6 + size;

        // Generate workspace.
        let mut g = Graph::with_size(space);

        // The two carbon atoms that are always used;
        g.atoms_mut()[0] = 6;
        g.atoms_mut()[1] = 6;
        *g.bonds_mut().get_mut(0, 1) = 1;
        *g.bonds_mut().get_mut(1, 0) = 1;

        // The last two added carbon atoms.
        let mut x = 0;
        let mut y = 1;

        for i in 0..size {
            let i = i + 2;

            // Add a triangle and atom.
            g.atoms_mut()[i] = 6;
            *g.bonds_mut().get_mut(x, i) = 1;
            *g.bonds_mut().get_mut(i, x) = 1;
            *g.bonds_mut().get_mut(y, i) = 1;
            *g.bonds_mut().get_mut(i, y) = 1;

            if x < y {
                x = i;
            } else {
                y = i;
            }
        }

        // First free id for hydrogen.
        let mut z = if x < y { y + 1 } else { x + 1};

        if x > y {
            std::mem::swap(&mut x, &mut y);
        }

        // Since 0 is the smaller ID it gains two hydrogen and 1 gains one.
        g.atoms_mut()[z] = 1;
        *g.bonds_mut().get_mut(0, z) = 1;
        *g.bonds_mut().get_mut(z, 0) = 1;
        z += 1;

        g.atoms_mut()[z] = 1;
        *g.bonds_mut().get_mut(0, z) = 1;
        *g.bonds_mut().get_mut(z, 0) = 1;
        z += 1;

        g.atoms_mut()[z] = 1;
        *g.bonds_mut().get_mut(1, z) = 1;
        *g.bonds_mut().get_mut(z, 1) = 1;
        z += 1;

        // Since x is now smaller, it gains one hydrogen and y gains two.
        g.atoms_mut()[z] = 1;
        *g.bonds_mut().get_mut(x, z) = 1;
        *g.bonds_mut().get_mut(z, x) = 1;
        z += 1;

        g.atoms_mut()[z] = 1;
        *g.bonds_mut().get_mut(y, z) = 1;
        *g.bonds_mut().get_mut(z, y) = 1;
        z += 1;

        g.atoms_mut()[z] = 1;
        *g.bonds_mut().get_mut(y, z) = 1;
        *g.bonds_mut().get_mut(z, y) = 1;
        
        assert_eq!(z + 1, space);

        g
    }
}

#[test]
fn dna_esque() {
    use super::MoleculeGenerator;

    for i in 0..10 {
        let g = DnaEsque::generate(i);
        // use std::fs::File;
        // use std::io::Write;   
        // let filename = format!("trace/dna_{}.dot", i);
        // let mut f = File::create(filename).unwrap();
        // writeln!(&mut f, "graph dna {{").unwrap();
        // g.dump(&mut f, 0, true).unwrap();
        // writeln!(&mut f, "}}").unwrap();

        assert!(g.is_contiguous());
    }
}