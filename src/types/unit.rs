use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum Units {
    Unitless,
    Metre,
    Second,
    KiloGram,
    Ampere,
    Kelvin,
    Mole,
    Candela,
}
#[allow(clippy::module_name_repetitions)]
pub struct UnitProduct {
    unit: HashMap<Units, i8>,
}

impl UnitProduct {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            unit: HashMap::new(),
        }
    }
    #[allow(non_snake_case)]
    pub fn to_SI(&self) {}
}

impl std::ops::Mul<Units> for Units {
    type Output = UnitProduct;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut prod = UnitProduct::new();
        prod.unit.insert(self, 1);
        prod.unit.entry(rhs).and_modify(|e| *e -= 1).or_insert(-1);
        prod
    }
}

impl std::ops::Div<Units> for Units {
    type Output = UnitProduct;
    fn div(self, rhs: Self) -> Self::Output {
        let mut prod = UnitProduct::new();
        prod.unit.insert(self, -1);
        prod.unit.entry(rhs).and_modify(|e| *e -= 1).or_insert(-1);
        prod
    }
}
