mod base_case;
mod count;
mod iteration;

#[cfg(test)]
mod test;

#[cfg(test)]
mod bench;

pub use self::base_case::subgraphs3;
pub use self::count::count_subgraphs;
pub use self::iteration::subgraphs;
pub use self::iteration::combine;
