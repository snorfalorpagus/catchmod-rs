pub struct LinearStore {
    pub constant: f64,

    pub previous_outflow: f64,
}

impl LinearStore {
    pub fn step(&mut self, inflow: f64) -> f64 {
        let b = if self.constant > 0.0 {
            (-1.0 / self.constant).exp()
        } else {
            0.0
        };

        let average_outflow = inflow - self.constant * (inflow - self.previous_outflow) * (1.0 - b);

        self.previous_outflow = if self.previous_outflow < 1e-9 {
            1e-8
        } else {
            inflow - (inflow - self.previous_outflow) * b
        };

        average_outflow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_constant() {
        let mut store = LinearStore { constant: 30.0, previous_outflow: 0.0 };
        let average_outflow = store.step(10.0);
        assert!(abs_diff_eq!(average_outflow, 0.1648, epsilon = 0.0001)) // TODO: check result
    }

    #[test]
    fn test_without_constant() {
        let mut store = LinearStore { constant: 0.0, previous_outflow: 0.0 };
        let average_outflow = store.step(10.0);
        assert!(abs_diff_eq!(average_outflow, 10.0, epsilon = 0.0001)) // TODO: check result
    }
}
