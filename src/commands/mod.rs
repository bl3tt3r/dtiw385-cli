pub mod infos;
pub use infos::*;

pub trait Merge {
    fn merge(&mut self, other: Self);
}
