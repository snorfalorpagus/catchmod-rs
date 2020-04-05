use crate::subcatchment::Subcatchment;

pub struct Catchment<'a> {
    pub subcatchments: Vec<Subcatchment<'a>>,
}

impl Catchment<'_> {
    pub fn step(&mut self, rainfall: f64, pet: f64) -> f64 {
        let mut outflow: f64 = 0.0;
        for subcatchment in self.subcatchments.iter_mut() {
            outflow += subcatchment.step(rainfall, pet);
        }
        outflow
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::linear::LinearStore;
    use crate::nonlinear::NonLinearStore;
    use crate::soil::SoilMoistureDeficitStore;

    #[test]
    fn test_catchment() {
        let subcatchment = Subcatchment {
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
        let mut subcatchments = Vec::new();
        subcatchments.push(subcatchment);
        let mut catchment = Catchment {
            subcatchments: subcatchments,
        };
        let outflow = catchment.step(50.0, 15.0);
        // println!("{}", outflow);
        assert!(abs_diff_eq!(outflow, 35.0, epsilon = 0.0001));
    }
}
