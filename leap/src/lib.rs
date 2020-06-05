trait DivisibleBy<T = Self> {
    fn divisible_by(self, rhs: T) -> bool;
}

impl DivisibleBy for u64 {
    fn divisible_by(self, rhs: u64) -> bool {
        self % rhs == 0
    }
}

pub fn is_leap_year(year: u64) -> bool {
    return match year {
        x if x.divisible_by(400) => true,
        x if x.divisible_by(100) => false,
        x => x.divisible_by(4),
    };
}
