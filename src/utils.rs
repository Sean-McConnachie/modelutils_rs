use super::F;

#[derive(Debug, Clone)]
pub struct MinMax {
    pub min: F,
    pub max: F,
}

impl MinMax {
    pub fn new(v1: F, v2: F) -> Self {
        if v1 < v2 {
            Self { min: v1, max: v2 }
        } else {
            Self { min: v2, max: v1 }
        }
    }

    pub fn update(&mut self, v: F) {
        if v > self.max {
            self.max = v;
        } else if v < self.min {
            self.min = v;
        }
    }

    pub fn range(&self) -> F {
        self.max - self.min
    }
}
