mod base_case;
pub mod variants;
mod count;
mod iteration;
mod r#trait;

#[cfg(test)]
mod test;

#[cfg(test)]
mod bench;

pub use self::base_case::subgraphs3;
pub use self::count::count_subgraphs;
pub use self::iteration::combine;
pub use self::iteration::subgraphs;
pub use self::r#trait::Subgraphs;
