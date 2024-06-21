pub trait MathyObject {
    fn namespace(&self) -> &'static str;
    fn name(&self) -> &'static str;
}

pub struct Symbol {}

impl MathyObject for Symbol {
    fn namespace(&self) -> &'static str {
        "math"
    }

    fn name(&self) -> &'static str {
        "frank"
    }
}
