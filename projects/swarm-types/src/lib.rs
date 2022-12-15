use diagnostic_quick::error_3rd::{BigInt, Decimal, NodeLocation};

pub use self::{
    errors::{LarvaError, SwarmResult},
    typing::LarvaType,
    vm::LarvaVM,
};

mod errors;
mod typing;
mod vm;

pub type LarvaNode = NodeLocation<LarvaKind>;

pub enum LarvaKind {
    String(String),
    Secret(String),
    Integer(BigInt),
    Decimal(Decimal),
}
