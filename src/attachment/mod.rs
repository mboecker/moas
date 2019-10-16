mod interface;
mod mapping;
mod perform;
mod result;

pub use self::interface::attach;
pub use self::perform::perform;
pub use self::result::Result;

#[cfg(test)]
mod test;
