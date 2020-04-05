use crate::linear::LinearStore;
use crate::nonlinear::NonLinearStore;
use crate::soil::SoilMoistureDeficitStore;

pub struct Subcatchment<'a> {
    pub name: &'a str,
    pub soil: &'a mut SoilMoistureDeficitStore,
    pub linear: &'a mut LinearStore,
    pub nonlinear: &'a mut NonLinearStore,
}

impl Subcatchment<'_> {
    pub fn step(&mut self, rainfall: f64, pet: f64) -> f64 {
        let percolation = self.soil.step(rainfall, pet);
        let linear_outflow = self.linear.step(percolation);
        let nonlinear_outflow = self.nonlinear.step(linear_outflow);
        nonlinear_outflow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subcatchment() {
        let mut c = Subcatchment {
            name: "test",
            soil: &mut SoilMoistureDeficitStore {
                direct_percolation: 1.0,
                potential_drying_constant: 0.0,
                gradient_drying_curve: 0.0,
                upper_deficit: 0.0,
                lower_deficit: 0.0,
            },
            linear: &mut LinearStore {
                constant: 0.0,
                previous_outflow: 0.0,
            },
            nonlinear: &mut NonLinearStore {
                constant: 0.0,
                previous_outflow: 0.0,
            },
        };
        let outflow = c.step(50.0, 15.0);
        assert!(abs_diff_eq!(outflow, 35.0, epsilon = 0.0001))
    }
}
