pub mod bool;
pub mod list;
pub mod num;

mod seal {
    pub trait Sealed {}
    pub struct Key {}
    impl Sealed for Key {}
}
pub struct Invalid;
impl seal::Sealed for Invalid {}
