use std::{fmt::Display, ops::Index};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum DieRoll {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl Display for DieRoll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DieRoll::One => write!(f, "⚀"),
            DieRoll::Two => write!(f, "⚁"),
            DieRoll::Three => write!(f, "⚂"),
            DieRoll::Four => write!(f, "⚃"),
            DieRoll::Five => write!(f, "⚄"),
            DieRoll::Six => write!(f, "⚅"),
        }
    }
}

const DIE_ROLLS: [DieRoll; 6] = [
    DieRoll::One,
    DieRoll::Two,
    DieRoll::Three,
    DieRoll::Four,
    DieRoll::Five,
    DieRoll::Six,
];

const SMALL_STRAIGHTS: [[DieRoll; 4]; 3] = {
    use DieRoll::*;
    [
        [One, Two, Three, Four],
        [Two, Three, Four, Five],
        [Three, Four, Five, Six],
    ]
};

const LARGE_STRAIGHTS: [[DieRoll; 5]; 2] = {
    use DieRoll::*;
    [[One, Two, Three, Four, Five], [Two, Three, Four, Five, Six]]
};

type DiceRolls = [DieRoll; 5];

#[derive(Debug)]
struct DiceCounts {
    ones: u16,
    twos: u16,
    threes: u16,
    fours: u16,
    fives: u16,
    sixes: u16,
}

impl DiceCounts {
    fn new(dice: [DieRoll; 5]) -> Self {
        let (mut ones, mut twos, mut threes, mut fours, mut fives, mut sixes) = (0, 0, 0, 0, 0, 0);
        for die in dice {
            match die {
                DieRoll::One => ones += 1,
                DieRoll::Two => twos += 1,
                DieRoll::Three => threes += 1,
                DieRoll::Four => fours += 1,
                DieRoll::Five => fives += 1,
                DieRoll::Six => sixes += 1,
            }
        }

        Self {
            ones,
            twos,
            threes,
            fours,
            fives,
            sixes,
        }
    }

    fn sum(&self) -> u16 {
        DIE_ROLLS.map(|die| self[die] * (die as u16)).iter().sum()
    }

    fn has_tuple(&self, n: u16) -> bool {
        for die in DIE_ROLLS {
            if self[die] >= n {
                return true;
            }
        }
        return false;
    }

    fn has_fullhouse(&self) -> bool {
        let mut has_pair = false;
        let mut has_triple = false;
        for die in DIE_ROLLS {
            if self[die] == 2 {
                has_pair = true;
            }
            if self[die] == 3 {
                has_triple = true;
            }
        }
        has_pair && has_triple
    }

    fn has_straight<const N: usize, const M: usize>(&self, straights: [[DieRoll; N]; M]) -> bool {
        straights
            .iter()
            .any(|straight| straight.iter().all(|&die| self[die] > 0))
    }

    fn has_small_straight(&self) -> bool {
        self.has_straight(SMALL_STRAIGHTS)
    }

    fn has_large_straight(&self) -> bool {
        self.has_straight(LARGE_STRAIGHTS)
    }

    fn times_die_values(&self) -> Self {
        Self {
            ones: self.ones * 1,
            twos: self.twos * 2,
            threes: self.threes * 3,
            fours: self.fours * 4,
            fives: self.fives * 5,
            sixes: self.sixes * 6,
        }
    }
}

impl Index<DieRoll> for DiceCounts {
    type Output = u16;

    fn index(&self, index: DieRoll) -> &Self::Output {
        match index {
            DieRoll::One => &self.ones,
            DieRoll::Two => &self.twos,
            DieRoll::Three => &self.threes,
            DieRoll::Four => &self.fours,
            DieRoll::Five => &self.fives,
            DieRoll::Six => &self.sixes,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Combination {
    Upper(DieRoll),
    Triple,
    Quadruple,
    Quintuple,
    SmallStraight,
    LargeStraight,
    Chance,
    FullHouse,
}

const LOWER_COMBINATIONS: [Combination; 7] = {
    use Combination::*;
    [
        Triple,
        Quadruple,
        SmallStraight,
        LargeStraight,
        FullHouse,
        Quintuple,
        Chance,
    ]
};

impl Display for Combination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Combination::Upper(number) => f.pad(&format!("{number} s")),
            Combination::Triple => f.pad("3 of a kind"),
            Combination::Quadruple => f.pad("4 of a kind"),
            Combination::Quintuple => f.pad("5 of a kind"),
            Combination::SmallStraight => f.pad("small straight"),
            Combination::LargeStraight => f.pad("large straight"),
            Combination::FullHouse => f.pad("full house"),
            Combination::Chance => f.pad("chance"),
        }
    }
}

#[derive(Debug)]
struct PotentialValues {
    upper: DiceCounts,
    triple: u16,
    quadruple: u16,
    quintuple: u16,
    small_straight: u16,
    large_straight: u16,
    full_house: u16,
    chance: u16,
}

impl PotentialValues {
    fn new(counts: DiceCounts) -> Self {
        Self {
            upper: counts.times_die_values(),
            triple: if counts.has_tuple(3) { counts.sum() } else { 0 },
            quadruple: if counts.has_tuple(4) { counts.sum() } else { 0 },
            quintuple: if counts.has_tuple(5) { 50 } else { 0 },
            chance: counts.sum(),
            small_straight: if counts.has_small_straight() { 30 } else { 0 },
            large_straight: if counts.has_large_straight() { 40 } else { 0 },
            full_house: if counts.has_fullhouse() { 25 } else { 0 },
        }
    }
}

impl Index<Combination> for PotentialValues {
    type Output = u16;

    fn index(&self, index: Combination) -> &Self::Output {
        match index {
            Combination::Upper(number) => &self.upper[number],
            Combination::Triple => &self.triple,
            Combination::Quadruple => &self.quadruple,
            Combination::Quintuple => &self.quintuple,
            Combination::SmallStraight => &self.small_straight,
            Combination::LargeStraight => &self.large_straight,
            Combination::Chance => &self.chance,
            Combination::FullHouse => &self.full_house,
        }
    }
}

#[derive(Clone, Copy)]
struct ValuedCombination {
    combination: Combination,
    value: u16,
}

impl Display for ValuedCombination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ValuedCombination { combination, value } = self;
        write!(f, "{value:2} for {combination:10}")
    }
}

struct Score {
    upper: u16,
    lower: u16,
    bonus: u16,
}

impl Score {
    fn total(&self) -> u16 {
        self.upper + self.lower + self.bonus
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Score {
            upper,
            lower,
            bonus,
        } = self;
        let total = self.total();
        write!(
            f,
            "Upper: {upper:3} Bonus: {bonus:2} Lower: {lower:3} Total: {total:3}"
        )
    }
}

struct PlayerState {
    filled: Vec<ValuedCombination>,
}

impl PlayerState {
    fn new() -> Self {
        PlayerState { filled: Vec::new() }
    }

    fn has_combination(&self, combination: Combination) -> bool {
        self.filled.iter().any(|vc| vc.combination == combination)
    }

    fn record_value(&mut self, vc: ValuedCombination) -> Result<(), &'static str> {
        if self.has_combination(vc.combination) {
            return Err("combination already recorded");
        } else {
            self.filled.push(vc);
            Ok(())
        }
    }

    fn score(&self) -> Score {
        let mut upper = 0;
        let mut lower = 0;
        for ValuedCombination { combination, value } in &self.filled {
            match combination {
                Combination::Upper(_) => upper += value,
                _ => lower += value,
            }
        }

        let bonus = if upper >= 63 { 35 } else { 0 };

        Score {
            upper,
            lower,
            bonus,
        }
    }

    fn is_done(&self) -> bool {
        self.filled.len() >= DIE_ROLLS.len() + LOWER_COMBINATIONS.len()
    }

    fn display(&self, term: &console::Term) -> std::io::Result<()> {
        for die in DIE_ROLLS {
            let combination = Combination::Upper(die);
            if let Some(ValuedCombination { combination, value }) =
                self.filled.iter().find(|vc| vc.combination == combination)
            {
                println!(
                    "{:15} ({value:2})",
                    console::style(combination).strikethrough()
                );
            } else {
                println!("{combination:15}     ");
            }
        }
        for (i, &combination) in LOWER_COMBINATIONS.iter().enumerate() {
            term.move_cursor_to(23, i)?;
            if let Some(ValuedCombination { combination, value }) =
                self.filled.iter().find(|vc| vc.combination == combination)
            {
                println!(
                    "{:15} ({value:2})",
                    console::style(combination).strikethrough()
                );
            } else {
                println!("{combination:15}     ");
            }
        }
        println!();
        println!("{}", self.score());
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let term = console::Term::stdout();
    let mut player_state = PlayerState::new();
    'outer: loop {
        let mut dice: DiceRolls = std::array::from_fn(|_| DIE_ROLLS[fastrand::usize(..6)]);
        let mut valued_combinations = Vec::new();
        let mut i = 0;
        loop {
            term.clear_screen()?;
            player_state.display(&term)?;
            if player_state.is_done() {
                break 'outer;
            }
            i += 1;
            dice.sort();
            println!();
            print!("You rolled:");
            for die in dice {
                print!(" {die}");
            }
            println!();
            let counts = DiceCounts::new(dice);
            let values = PotentialValues::new(counts);
            valued_combinations.clear();
            for number in DIE_ROLLS {
                let combination = Combination::Upper(number);
                valued_combinations.push(ValuedCombination {
                    combination,
                    value: values[combination],
                });
            }
            for combination in LOWER_COMBINATIONS {
                valued_combinations.push(ValuedCombination {
                    combination,
                    value: values[combination],
                });
            }
            valued_combinations.retain(|vc| !player_state.has_combination(vc.combination));
            valued_combinations.sort_by_key(|vc| 100 - vc.value);
            if i > 2 {
                break;
            }
            for vc in &valued_combinations {
                if vc.value == 0 {
                    break;
                }
                println!("{vc}");
            }
            let selection = dialoguer::MultiSelect::new()
                .with_prompt("Select the dice that you want to roll again")
                .items(&dice)
                .interact()
                .unwrap();
            if selection.is_empty() {
                break;
            }
            for idx in selection {
                dice[idx] = DIE_ROLLS[fastrand::usize(..6)];
            }
        }
        let selection = dialoguer::Select::new()
            .with_prompt("What combination do you want to record?")
            .items(&valued_combinations)
            .interact()
            .unwrap();
        player_state
            .record_value(valued_combinations[selection])
            .expect("recorded combination should not have been selectable");
    }

    Ok(())
}
