const ZERO: f64 = 1e-9;
const T: f64 = 1.0;

pub struct NonLinearStore {
    pub constant: f64,

    pub previous_outflow: f64,
}

impl NonLinearStore {
    pub fn step(&mut self, inflow: f64) -> f64 {
        if self.constant < ZERO {
            return inflow;
        }

        let q2 = if self.previous_outflow > 0.0 {
            if inflow < -ZERO {
                self.case_b(inflow)
            } else if inflow > ZERO {
                self.case_c(inflow)
            } else {
                self.case_a()
            }
        } else {
            self.case_d(inflow)
        };

        (self.previous_outflow + q2) / 2.0
    }

    fn case_a(&mut self) -> f64 {
        self.constant / ((self.constant / self.previous_outflow).sqrt() + T).powi(2)
    }

    fn case_b(&mut self, inflow: f64) -> f64 {
        let a = (-self.previous_outflow / inflow).sqrt().atan() - (-inflow / self.constant).sqrt();
        if a > 0.0 {
            return -inflow * a.tan().powi(2);
        } else {
            return 0.0;
        }
    }

    fn case_c(&mut self, inflow: f64) -> f64 {
        let a =
            (self.previous_outflow.sqrt() - inflow.sqrt()) / (self.previous_outflow.sqrt() + inflow.sqrt());
        let b = -2.0 * T * (inflow / self.constant).sqrt();
        inflow * ((1.0 + a * b.exp()) / (1.0 - a * b.exp())).powi(2)
    }

    fn case_d(&mut self, inflow: f64) -> f64 {
        let v = -(self.previous_outflow.abs() * self.constant).sqrt() + inflow * T;
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
        let mut store = NonLinearStore { constant: 0.0, previous_outflow: 0.0 };
        let outflow = store.step(999.0);
        assert!(abs_diff_eq!(outflow, 999.0, epsilon = 0.0001))
    }

    #[test]
    fn near_zero_constant() {
        let mut store = NonLinearStore { constant: 1e-12, previous_outflow: 0.0 };
        let outflow = store.step(999.0);
        assert!(abs_diff_eq!(outflow, 999.0, epsilon = 0.0001))
    }

    #[test]
    fn case_d() {
        let mut store = NonLinearStore { constant: 30.0, previous_outflow: 10.0 };
        let outflow = store.step(50.0);
        assert!(abs_diff_eq!(outflow, 27.27135, epsilon = 0.0001))
    }
}
