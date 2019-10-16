mod base_case;
mod count;
mod iteration;
mod r#trait;
pub mod variants;

#[cfg(test)]
mod test;

#[cfg(test)]
mod bench;

pub use self::base_case::subgraphs3;
pub use self::count::count_subgraphs;
pub use self::iteration::combine;
pub use self::iteration::get_all;
pub use self::r#trait::Subgraphs;
