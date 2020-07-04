pub struct Allergies(u32);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergen {
    fn from_u32(value: u32) -> Allergen {
        match value {
            1 => Allergen::Eggs,
            2 => Allergen::Peanuts,
            4 => Allergen::Shellfish,
            8 => Allergen::Strawberries,
            16 => Allergen::Tomatoes,
            32 => Allergen::Chocolate,
            64 => Allergen::Pollen,
            128 => Allergen::Cats,
            _ => unreachable!("there's no allergen for given number"),
        }
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0 & (*allergen as u32) == *allergen as u32
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        (0..=7)
            .filter_map(|n| {
                let allergie = Allergen::from_u32(1 << n);
                if self.is_allergic_to(&allergie) {
                    return Some(allergie);
                }
                None
            })
            .collect()
    }
}
