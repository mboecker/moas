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

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn fill_new(a: usize, default: T) -> Matrix<T> {
        Matrix {
            d: (0..a * a).map(|_| default.clone()).collect(),
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

#[allow(dead_code)]
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

pub fn get_database_similar(conn: &rusqlite::Connection, id: String) -> Vec<(usize, Graph)> {
    // Select similar graphs from database.
    let sql = format!("SELECT hashes.cid, structure FROM hashes INNER JOIN compounds ON hashes.cid = compounds.cid WHERE hash = ?");
    let mut stmt = conn.prepare(&sql).unwrap();
    let iter = stmt
        .query_map(&[&id], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })
        .unwrap()
        .map(|x| x.unwrap());

    iter.map(|(cid, structure)| (cid as usize, crate::Graph::from_json(structure)))
        .collect()
}
