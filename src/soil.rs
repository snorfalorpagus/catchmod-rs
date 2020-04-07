pub struct SoilMoistureDeficitStore {
    pub direct_percolation: f64,
    pub potential_drying_constant: f64,
    pub gradient_drying_curve: f64,

    pub upper_deficit: f64,
    pub lower_deficit: f64,
}

impl SoilMoistureDeficitStore {
    pub fn step(&mut self, rainfall: f64, pet: f64) -> f64 {
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
    fn drying() {
        // When PET is greater than rainfall the effective rainfall is
        // negative and drying occurs
        let mut soil = SoilMoistureDeficitStore {
            direct_percolation: 1.0,
            potential_drying_constant: 70.0,
            gradient_drying_curve: 0.5,
            upper_deficit: 0.0,
            lower_deficit: 0.0,
        };
        // The upper storage dries first
        let percolation = soil.step(50.0, 100.0);
        assert!(abs_diff_eq!(percolation, 0.0, epsilon = 0.0001));
        assert!(abs_diff_eq!(soil.upper_deficit, 50.0, epsilon = 0.0001));
        assert!(abs_diff_eq!(soil.lower_deficit, 0.0, epsilon = 0.0001));
        // Once the threshold of the upper storage is reached the lower storage
        // begins to dry, at a rate defined by the gradient drying curve
        let percolation = soil.step(50.0, 100.0);
        assert!(abs_diff_eq!(percolation, 0.0, epsilon = 0.0001));
        assert!(abs_diff_eq!(soil.upper_deficit, 70.0, epsilon = 0.0001));
        assert!(abs_diff_eq!(soil.lower_deficit, 15.0, epsilon = 0.0001));
    }

    #[test]
    fn wetting() {
        let mut soil = SoilMoistureDeficitStore {
            direct_percolation: 0.0,
            potential_drying_constant: 15.0,
            gradient_drying_curve: 1.0,
            upper_deficit: 15.0,
            lower_deficit: 15.0,
        };
        // Upper deficit is satisfied first
        let percolation = soil.step(10.0, 0.0);
        assert!(abs_diff_eq!(percolation, 0.0, epsilon = 0.0001));
        assert!(abs_diff_eq!(soil.upper_deficit, 5.0, epsilon = 0.0001));
        assert!(abs_diff_eq!(soil.lower_deficit, 15.0, epsilon = 0.0001));
        // Lower deficit is satisfied second
        let percolation = soil.step(10.0, 0.0);
        assert!(abs_diff_eq!(percolation, 0.0, epsilon = 0.0001));
        assert!(abs_diff_eq!(soil.upper_deficit, 0.0, epsilon = 0.0001));
        assert!(abs_diff_eq!(soil.lower_deficit, 10.0, epsilon = 0.0001));
        // Remaining flow after satisfying deficit percolates
        let percolation = soil.step(15.0, 0.0);
        assert!(abs_diff_eq!(percolation, 5.0, epsilon = 0.0001));
        assert!(abs_diff_eq!(soil.upper_deficit, 0.0, epsilon = 0.0001));
        assert!(abs_diff_eq!(soil.lower_deficit, 0.0, epsilon = 0.0001));
    }

    #[test]
    fn direct_percolation_bypasses_stores() {
        let mut soil = SoilMoistureDeficitStore {
            direct_percolation: 1.0,
            potential_drying_constant: 100.0,
            gradient_drying_curve: 1.0,
            upper_deficit: 100.0,
            lower_deficit: 100.0,
        };
        // When direct percolation is 1 all of the rainfall bypasses the store
        let percolation = soil.step(50.0, 0.0);
        assert!(abs_diff_eq!(percolation, 50.0, epsilon = 0.0001));
        assert!(abs_diff_eq!(soil.upper_deficit, 100.0, epsilon = 0.0001));
        assert!(abs_diff_eq!(soil.lower_deficit, 100.0, epsilon = 0.0001));
        // When direct percolation is 0.5, half bypasses the store while the
        // other half must first satisfy the deficit
        soil.direct_percolation = 0.5;
        let percolation = soil.step(50.0, 0.0);
        assert!(abs_diff_eq!(percolation, 25.0, epsilon = 0.0001));
        assert!(abs_diff_eq!(soil.upper_deficit, 75.0, epsilon = 0.0001));
        assert!(abs_diff_eq!(soil.lower_deficit, 100.0, epsilon = 0.0001));
    }
}
