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

impl UnitProduct{
    pub fn new() -> UnitProduct {
        UnitProduct {
            unit: HashMap::new(),
        }
    }
    pub fn to_SI(self) {}
}

impl std::ops::Mul<Units> for Units {
    type Output = UnitProduct;
    fn mul(self, rhs: Units) -> Self::Output {
        let mut prod = UnitProduct::new();
        prod.unit.insert(self, 1);
        if prod.unit.contains_key(&rhs) {
            prod.unit[&rhs] += 1;
        } else {
            prod.unit.insert(rhs, 1);
        }
        prod
    }
}

impl std::ops::Div<Units> for Units {
    type Output = UnitProduct;
    fn div(self, rhs: Units) -> Self::Output {
        let mut prod = UnitProduct::new();
        prod.unit.insert(self, -1);
        if prod.unit.contains_key(&rhs) {
            prod.unit[&rhs] -= 1;
        } else {
            prod.unit.insert(rhs, -1);
        }
        prod
    }
}
