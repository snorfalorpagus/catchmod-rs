pub struct SoilMoistureDeficitStore {
    direct_percolation: f64,
    potential_drying_constant: f64,
    gradient_drying_curve: f64,
}

impl SoilMoistureDeficitStore {
    pub fn step(&self, rainfall: f64, pet: f64, upper_deficit: f64, lower_deficit: f64) -> (f64, f64, f64) {
        let mut effective_rainfall = rainfall - pet;
        let mut percolation = 0.0;
        let mut new_upper_deficit: f64 = upper_deficit;
        let mut new_lower_deficit: f64 = lower_deficit;

        if effective_rainfall > 0.0 {
            // Wetting
            // First calculate direct percolation, this proportion bypasses the store entirely            
            let direct_percolation = effective_rainfall * self.direct_percolation;
            percolation += direct_percolation;
            effective_rainfall -= direct_percolation;

            if new_upper_deficit > effective_rainfall {
                new_upper_deficit -= effective_rainfall;
                effective_rainfall = 0.0;
            } else if new_upper_deficit > 0.0 {
                effective_rainfall -= new_upper_deficit;
                new_upper_deficit = 0.0;
            }

            if effective_rainfall > 0.0 {
                if new_lower_deficit > effective_rainfall {
                    new_lower_deficit -= effective_rainfall;
                    effective_rainfall = 0.0;
                } else if new_lower_deficit > 0.0 {
                    effective_rainfall -= new_lower_deficit;
                    new_lower_deficit = 0.0;
                }
            }

            // If there is still any remaining effective rainfall, then it is saturated percolation
            if effective_rainfall > 0.0 {
                percolation += effective_rainfall
            }
        } else {
            // Drying
            if new_upper_deficit < self.potential_drying_constant + effective_rainfall {
                // Upper deficit sufficiently less than the PDC threshold, just increase the deficit
                new_upper_deficit -= effective_rainfall;
                effective_rainfall = 0.0;
            } else if new_upper_deficit < self.potential_drying_constant {
                // Upper deficit near to PDC threshold
                effective_rainfall += self.potential_drying_constant - new_upper_deficit;
                new_upper_deficit = self.potential_drying_constant
            }
            // If there is remaining negative effective rainfall dry the lower store at reduced rate
            if effective_rainfall < 0.0 {
                // There is no limit to the size of the lower store
                new_lower_deficit -= effective_rainfall * self.gradient_drying_curve;
            }
        }

        (percolation, new_upper_deficit, new_lower_deficit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_storage() {
        let soil = SoilMoistureDeficitStore{ direct_percolation: 1.0, potential_drying_constant: 0.0, gradient_drying_curve: 0.0};
        let (percolation, _, _) = soil.step(50.0, 15.0, 0.0, 0.0);
        assert!(abs_diff_eq!(percolation, 35.0, epsilon = 0.0001))
    }

    #[test]
    fn negative_effective_rainfall() {
        // When PET > rainfall there is zero percolation
        let soil = SoilMoistureDeficitStore{ direct_percolation: 1.0, potential_drying_constant: 0.0, gradient_drying_curve: 0.0};
        let (percolation, _, _) = soil.step(50.0, 100.0, 0.0, 0.0);
        assert!(abs_diff_eq!(percolation, 0.0, epsilon = 0.0001))
    }
}
