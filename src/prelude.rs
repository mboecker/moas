use crate::Graph;
use serde::Serialize;
use std::io::Write;
use std::iter::Sum;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct Matrix<T> {
    d: Vec<T>,
    size: usize,
}

impl<T> Matrix<T>
where
    T: Default,
{
    pub fn new(a: usize) -> Matrix<T> {
        Matrix {
            d: (0..a * a).map(|_| T::default()).collect(),
            size: a,
        }
    }
}

impl<T> Matrix<T> {
    pub fn get(&self, i: usize, j: usize) -> &T {
        &self.d[i + j * self.size]
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut T {
        &mut self.d[i + j * self.size]
    }
}

impl Matrix<u8> {
    pub fn sum(&self) -> usize {
        self.d.iter().map(|x| *x as usize).sum()
    }
}

pub fn dump_set<'a>(
    mut out: impl Write,
    set: impl Iterator<Item = &'a Graph>,
) -> std::io::Result<()> {
    let mut i = 0;
    writeln!(out, "graph set {{")?;
    for g in set {
        g.dump(&mut out, i, true)?;
        i += g.size();
    }
    writeln!(out, "}}")?;
    std::io::Result::Ok(())
}
