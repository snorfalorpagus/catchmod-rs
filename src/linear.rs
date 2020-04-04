pub struct LinearStore {
    constant: f64,
}

impl LinearStore {
    pub fn step(&self, inflow: f64, previous_outflow: f64) -> (f64, f64) {
        let b = if self.constant > 0.0 {
            (-1.0 / self.constant).exp()
        } else {
            0.0
        };

        let average_outflow = inflow - self.constant * (inflow - previous_outflow) * (1.0 - b);

        let outflow = if previous_outflow < 1e-9 {
            1e-8
        } else {
            inflow - (inflow - previous_outflow) * b
        };

        return (average_outflow, outflow);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_constant() {
        let store = LinearStore{ constant: 30.0 };
        let (average_outflow, _) = store.step(10.0, 0.0);
        assert!(abs_diff_eq!(average_outflow, 0.1648, epsilon = 0.0001))  // TODO: check result
    }

    #[test]
    fn test_without_constant() {
        let store = LinearStore{ constant: 0.0 };
        let (average_outflow, _) = store.step(10.0, 0.0);
        assert!(abs_diff_eq!(average_outflow, 10.0, epsilon = 0.0001))  // TODO: check result
    }
}
