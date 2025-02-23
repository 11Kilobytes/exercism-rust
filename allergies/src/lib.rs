#[derive(Copy, Clone)]
pub struct Allergies(u32);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        match self {
            Allergies(score) => ((2u32.pow(*allergen as u32)) & score) != 0,
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        use Allergen::*;
        [
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ]
        .into_iter()
        .filter(|x| self.is_allergic_to(x))
        .collect()
    }
}
