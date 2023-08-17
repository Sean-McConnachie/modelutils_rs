use super::float;

#[derive(Debug, Clone)]
pub struct MinMax {
    pub min: float,
    pub max: float,
}

impl MinMax {
    pub fn new(v1: float, v2: float) -> Self {
        if v1 < v2 {
            Self { min: v1, max: v2 }
        } else {
            Self { min: v2, max: v1 }
        }
    }

    pub fn update(&mut self, v: float) {
        if v > self.max {
            self.max = v;
        } else if v < self.min {
            self.min = v;
        }
    }

    pub fn range(&self) -> float {
        self.max - self.min
    }
}