pub(crate) struct Lcg {
    seed: rally_core::RunSeed,
}

impl Lcg {
    pub(crate) fn new(seed: u64) -> Self {
        Self {
            seed: rally_core::RunSeed::from_u64(seed),
        }
    }

    pub(crate) fn next_u32(&mut self, max: u32) -> u32 {
        self.seed.next_bounded(max)
    }

    pub(crate) fn percent(&mut self, chance: u32) -> bool {
        self.seed.percent_chance(chance)
    }
}

pub(crate) fn sample_between(rng: &mut Lcg, min: u32, max_inclusive: u32) -> u32 {
    rally_core::sample_between(&mut rng.seed, min, max_inclusive)
}

pub(crate) fn percent(numerator: u32, denominator: u32) -> f64 {
    rally_core::percent_of(numerator, denominator)
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
