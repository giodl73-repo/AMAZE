pub(crate) struct Lcg {
    state: u64,
}

impl Lcg {
    pub(crate) fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    pub(crate) fn next_u32(&mut self, max: u32) -> u32 {
        if max == 0 {
            return 0;
        }
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.state >> 32) as u32) % max
    }

    pub(crate) fn percent(&mut self, chance: u32) -> bool {
        self.next_u32(100) < chance.min(100)
    }
}

pub(crate) fn sample_between(rng: &mut Lcg, min: u32, max_inclusive: u32) -> u32 {
    if max_inclusive <= min {
        return min;
    }
    min + rng.next_u32(max_inclusive - min + 1)
}

pub(crate) fn percent(numerator: u32, denominator: u32) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 * 100.0 / denominator as f64
    }
}

#[cfg(test)]
mod tests {
    use super::{sample_between, Lcg};

    #[test]
    fn rng_is_deterministic() {
        let mut left = Lcg::new(42);
        let mut right = Lcg::new(42);
        assert_eq!(left.next_u32(100), right.next_u32(100));
        assert_eq!(left.next_u32(100), right.next_u32(100));
    }

    #[test]
    fn samples_between_inclusive_bounds() {
        let mut rng = Lcg::new(1);
        for _ in 0..20 {
            let sample = sample_between(&mut rng, 2, 4);
            assert!((2..=4).contains(&sample));
        }
    }
}
