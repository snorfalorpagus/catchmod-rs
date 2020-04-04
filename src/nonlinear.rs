const ZERO: f64 = 1e-9;
const T: f64 = 1.0;

pub struct NonLinearStore {
    pub constant: f64,
}

impl NonLinearStore {
    pub fn step(&self, inflow: f64, previous_outflow: f64) -> (f64, f64) {
        if self.constant < ZERO {
            return (inflow, previous_outflow);
        }

        let q2 = if previous_outflow > 0.0 {
            if inflow < -ZERO {
                self.case_b(inflow, previous_outflow)
            } else if inflow > ZERO {
                self.case_c(inflow, previous_outflow)
            } else {
                self.case_a(previous_outflow)
            }
        } else {
            self.case_d(inflow, previous_outflow)
        };

        return ((previous_outflow + q2) / 2.0, q2);
    }

    fn case_a(&self, previous_outflow: f64) -> f64 {
        self.constant / ((self.constant / previous_outflow).sqrt() + T).powi(2)
    }

    fn case_b(&self, inflow: f64, previous_outflow: f64) -> f64 {
        let a = (-previous_outflow / inflow).sqrt().atan() - (-inflow / self.constant).sqrt();
        if a > 0.0 {
            return -inflow * a.tan().powi(2);
        } else {
            return 0.0;
        }
    }

    fn case_c(&self, inflow: f64, previous_outflow: f64) -> f64 {
        let a =
            (previous_outflow.sqrt() - inflow.sqrt()) / (previous_outflow.sqrt() + inflow.sqrt());
        let b = -2.0 * T * (inflow / self.constant).sqrt();
        inflow * ((1.0 + a * b.exp()) / (1.0 - a * b.exp())).powi(2)
    }

    fn case_d(&self, inflow: f64, previous_outflow: f64) -> f64 {
        let v = -(previous_outflow.abs() * self.constant).sqrt() + inflow * T;
        if v > 0.0 {
            let t = T - v / inflow;
            let a = 1.0;
            let b = -2.0 * (T - t) * (inflow / self.constant).sqrt();
            return inflow * ((1.0 - a * b.exp()) / (1.0 + a * b.exp())).powi(2);
        } else {
            return 0.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_constant() {
        let store = NonLinearStore { constant: 0.0 };
        let (outflow, _) = store.step(999.0, 0.0);
        assert!(abs_diff_eq!(outflow, 999.0, epsilon = 0.0001))
    }

    #[test]
    fn near_zero_constant() {
        let store = NonLinearStore { constant: 1e-12 };
        let (outflow, _) = store.step(999.0, 0.0);
        assert!(abs_diff_eq!(outflow, 999.0, epsilon = 0.0001))
    }

    #[test]
    fn case_d() {
        let store = NonLinearStore { constant: 0.3 };
        let (outflow, _) = store.step(50.0, 10.0);
        assert!(abs_diff_eq!(outflow, 29.9999, epsilon = 0.0001))
    }
}
