use crate::LarvaKind;

impl LarvaKind {
    pub fn r#type(&self) -> LarvaType {
        match self {
            LarvaKind::String(_) => LarvaType::String,
            LarvaKind::Secret(_) => LarvaType::Secret,
        }
    }
    pub fn cast(&mut self, r#type: LarvaType) -> QResult {
        match (self, r#type) {
            (LarvaKind::String(s), LarvaType::Secret) => {
                *self = LarvaKind::Secret(s.clone());
            }
            _ => {
                todo!()
            }
        }
        Ok(())
    }
}

pub enum LarvaType {
    String,
    Secret,
}
