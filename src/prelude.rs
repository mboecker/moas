use crate::Graph;
use serde::Serialize;
use std::io::Write;

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

    pub fn bytes(&self) -> usize {
        self.d.capacity() * std::mem::size_of::<T>()
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

pub fn dump_map<'a>(
    mut out: impl Write,
    set: impl Iterator<Item = (&'a Graph, &'a usize)>,
) -> std::io::Result<()> {
    let mut i = 0;
    writeln!(out, "graph set {{")?;
    for (g, v) in set {
        for _ in 0..*v {
            g.dump(&mut out, i, true)?;
            i += g.size();
        }
    }
    writeln!(out, "}}")?;
    std::io::Result::Ok(())
}
