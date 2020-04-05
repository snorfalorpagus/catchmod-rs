pub struct SoilMoistureDeficitStore {
    pub direct_percolation: f64,
    pub potential_drying_constant: f64,
    pub gradient_drying_curve: f64,
    
    pub upper_deficit: f64,
    pub lower_deficit: f64,
}

impl SoilMoistureDeficitStore {
    pub fn step(
        &mut self,
        rainfall: f64,
        pet: f64,
    ) -> f64 {
        let mut effective_rainfall = rainfall - pet;
        let mut percolation = 0.0;

        if effective_rainfall > 0.0 {
            // Wetting
            // First calculate direct percolation, this proportion bypasses the store entirely
            let direct_percolation = effective_rainfall * self.direct_percolation;
            percolation += direct_percolation;
            effective_rainfall -= direct_percolation;

            if self.upper_deficit > effective_rainfall {
                self.upper_deficit -= effective_rainfall;
                effective_rainfall = 0.0;
            } else if self.upper_deficit > 0.0 {
                effective_rainfall -= self.upper_deficit;
                self.upper_deficit = 0.0;
            }

            if effective_rainfall > 0.0 {
                if self.lower_deficit > effective_rainfall {
                    self.lower_deficit -= effective_rainfall;
                    effective_rainfall = 0.0;
                } else if self.lower_deficit > 0.0 {
                    effective_rainfall -= self.lower_deficit;
                    self.lower_deficit = 0.0;
                }
            }

            // If there is still any remaining effective rainfall, then it is saturated percolation
            if effective_rainfall > 0.0 {
                percolation += effective_rainfall
            }
        } else {
            // Drying
            if self.upper_deficit < self.potential_drying_constant + effective_rainfall {
                // Upper deficit sufficiently less than the PDC threshold, just increase the deficit
                self.upper_deficit -= effective_rainfall;
                effective_rainfall = 0.0;
            } else if self.upper_deficit < self.potential_drying_constant {
                // Upper deficit near to PDC threshold
                effective_rainfall += self.potential_drying_constant - self.upper_deficit;
                self.upper_deficit = self.potential_drying_constant
            }
            // If there is remaining negative effective rainfall dry the lower store at reduced rate
            if effective_rainfall < 0.0 {
                // There is no limit to the size of the lower store
                self.lower_deficit -= effective_rainfall * self.gradient_drying_curve;
            }
        }

        percolation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_storage() {
        let mut soil = SoilMoistureDeficitStore {
            direct_percolation: 1.0,
            potential_drying_constant: 0.0,
            gradient_drying_curve: 0.0,
            upper_deficit: 0.0,
            lower_deficit: 0.0,
        };
        let percolation = soil.step(50.0, 15.0);
        assert!(abs_diff_eq!(percolation, 35.0, epsilon = 0.0001))
    }

    #[test]
    fn negative_effective_rainfall() {
        // When PET > rainfall there is zero percolation
        let mut soil = SoilMoistureDeficitStore {
            direct_percolation: 1.0,
            potential_drying_constant: 0.0,
            gradient_drying_curve: 0.0,
            upper_deficit: 0.0,
            lower_deficit: 0.0,
        };
        let percolation = soil.step(50.0, 100.0);
        assert!(abs_diff_eq!(percolation, 0.0, epsilon = 0.0001))
    }
}
