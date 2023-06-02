pub struct Allergies(u32);

#[derive(Debug, PartialEq, Eq, Clone)]
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

use self::Allergen::*;
const ALLERGENS: [Allergen; 8] = [
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0 >> (allergen.clone() as u32) & 1 == 1
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS.iter().filter(|i| self.is_allergic_to(i)).cloned().collect::<Vec<_>>()
    }
}
