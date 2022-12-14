use diagnostic_quick::QResult;

use crate::LarvaKind;

impl LarvaKind {
    pub fn r#type(&self) -> LarvaType {
        match self {
            LarvaKind::String(_) => LarvaType::String,
            LarvaKind::Secret(_) => LarvaType::Secret,
            LarvaKind::Integer(_) => LarvaType::Integer,
            LarvaKind::Decimal(_) => LarvaType::Decimal,
        }
    }
    pub fn cast(&self, r#type: LarvaType) -> QResult<LarvaKind> {
        let out = match (self, r#type) {
            (LarvaKind::String(s), LarvaType::Secret) => LarvaKind::Secret(s.clone()),
            _ => {
                todo!()
            }
        };
        Ok(out)
    }
}

pub enum LarvaType {
    String,
    Secret,
    Integer,
    Decimal,
}
