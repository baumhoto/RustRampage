#[derive(Debug, Default, Clone, Copy)]
pub struct Rotation {
    pub m1: f64,
    pub m2: f64,
    pub m3: f64,
    pub m4: f64,
}

impl Rotation {
    pub fn new(sine: f64, cosine: f64) -> Self {
        Self {
            m1: cosine,
            m2: -sine,
            m3: sine,
            m4: cosine,
        }
    }
}
