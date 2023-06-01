pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];
pub fn score(mut dice: Dice, category: Category) -> u8 {
    dice.sort();
    match category {
        Category::Ones => dice.into_iter().filter(|i| *i == 1).count() as u8,
        Category::Twos => 2 * dice.into_iter().filter(|i| *i == 2).count() as u8,
        Category::Threes => 3 * dice.into_iter().filter(|i| *i == 3).count() as u8,
        Category::Fours => 4 * dice.into_iter().filter(|i| *i == 4).count() as u8,
        Category::Fives => 5 * dice.into_iter().filter(|i| *i == 5).count() as u8,
        Category::Sixes => 6 * dice.into_iter().filter(|i| *i == 6).count() as u8,
        Category::FullHouse if dice[0] == dice[1] && dice[1] == dice[2] && dice[2] != dice[3] && dice[3] == dice[4] => dice.into_iter().sum::<u8>(),
        Category::FullHouse if dice[0] == dice[1] && dice[1] != dice[2] && dice[2] == dice[3] && dice[3] == dice[4] => dice.into_iter().sum::<u8>(),
        Category::FourOfAKind if dice[0] == dice[1] && dice[1] == dice[2] && dice[2] == dice[3] => dice.into_iter().sum::<u8>() - dice[4],
        Category::FourOfAKind if dice[1] == dice[2] && dice[2] == dice[3] && dice[3] == dice[4] => dice.into_iter().sum::<u8>() - dice[0],
        Category::LittleStraight if dice.into_iter().enumerate().all(|(i, v)| v - i as u8 == 1) => 30,
        Category::BigStraight if dice.into_iter().enumerate().all(|(i, v)| v - i as u8 == 2) => 30,
        Category::Choice => dice.into_iter().sum::<u8>(),
        Category::Yacht if dice.into_iter().all(|i| i == dice[0]) => 50,
        _ => 0,
    }
}
