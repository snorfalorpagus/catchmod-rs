use crate::linear::LinearStore;
use crate::nonlinear::NonLinearStore;
use crate::soil::SoilMoistureDeficitStore;

pub struct Subcatchment<'a> {
    pub name: &'a str,
    soil: &'a SoilMoistureDeficitStore,
    linear: &'a LinearStore,
    nonlinear: &'a NonLinearStore,
}

impl Subcatchment<'_> {
    pub fn step(&self, rainfall: f64, pet: f64) -> f64 {
        let (percolation, upper_deficit, lower_deficit) = self.soil.step(rainfall, pet, 0.0, 0.0);
        let (linear_outflow, previous_outflow) = self.linear.step(percolation, 0.0);
        let (nonlinear_outflow, previous_outflow) = self.nonlinear.step(linear_outflow, 0.0);
        nonlinear_outflow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subcatchment() {
        let c = Subcatchment {
            name: "test",
            soil: &SoilMoistureDeficitStore {
                direct_percolation: 1.0,
                potential_drying_constant: 0.0,
                gradient_drying_curve: 0.0,
            },
            linear: &LinearStore { constant: 0.0 },
            nonlinear: &NonLinearStore { constant: 0.0 },
        };
        let outflow = c.step(50.0, 15.0);
        assert!(abs_diff_eq!(outflow, 35.0, epsilon = 0.0001))
    }
}
