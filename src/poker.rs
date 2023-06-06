#[derive(Debug, PartialEq)]
pub enum PokerCardSuit {
    Heart,
    Diamond,
    Clubs,
    Spades,
}

#[derive(Debug, PartialEq)]
pub struct PokerCards(PokerCardSuit, u8);

impl TryFrom<&str> for PokerCards {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let chars = value.chars().collect::<Vec<_>>();
        let rank = match chars.as_slice() {
            ['A', _] => 14,
            ['K', _] => 13,
            ['Q', _] => 12,
            ['J', _] => 11,
            ['T', _] => 10,
            [a, _] => a.to_digit(10).unwrap() as u8,
            _ => Err("Wrong number of character for a PokerCard.")?,
        };
        Ok(match chars.as_slice() {
            [_, 'H'] => Self(PokerCardSuit::Heart, rank),
            [_, 'D'] => Self(PokerCardSuit::Diamond, rank),
            [_, 'C'] => Self(PokerCardSuit::Clubs, rank),
            [_, 'S'] => Self(PokerCardSuit::Spades, rank),
            _ => Err("Not a known suit of PokerCard.")?,
        })
    }
}

/// Represents a hand in poker.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum PokerHand {
    /// Represents a hand with no special combination.
    /// Holds the highest card on hand.
    HighCard(u8),
    /// Represents a pair of cards of same value.
    /// First value is the value of the pair,
    /// the second is the highest card on hand.
    OnePair(u8, u8),
    /// Represents two pair of cards of same value.
    /// First value is the value of first the pair,
    /// the second is the value of first the pair,
    /// the third is the highest card on hand.
    TwoPairs(u8, u8, u8),
    /// Represents a triplet of cards of same value.
    /// First value is the value of the triplet,
    /// the second  is the highest card on hand.
    ThreeOfAKind(u8, u8),
    /// Represents a sequence of cards.
    /// First value is first value of the sequence,
    /// the second  is the highest card on hand.
    Straight(u8, u8),
    /// Represents a hand with all cards of the same suit.
    /// Holds the highest card on hand.
    Flush(u8),
    /// Represents a triplet plus a pair.
    /// First value is the value of the triplet,
    /// the second is the value of the pair.
    FullHouse(u8, u8),
    /// Represents a quadruplet of cards of same value.
    /// First value is the value of the quadruplet,
    /// the second is the highest card on hand.
    FourOfAKind(u8, u8),
    /// Represents a sequence of consecutive values and same suit.
    /// First value is first value of the sequence,
    /// the second is the highest card on hand.
    StraightFlush(u8, u8),
    /// Represents a Straight Flush starting at 10.
    RoyalFlush,
}

impl PokerHand {
    pub fn new(cards: &[PokerCards]) -> Result<Self, String> {
        match cards {
            [PokerCards(sa, ra), PokerCards(sb, rb), PokerCards(sc, rc), PokerCards(sd, rd), PokerCards(se, re)]
                if vec![sb, sc, sd, se].into_iter().all(|s| s == sa) =>
            {
                let mut values = vec![ra, rb, rc, rd, re];
                values.sort();
                let min = values[0];
                let max = values[4];
                if values.as_slice().windows(2).all(|wind| {
                    if let [a, b] = wind {
                        *a + 1 == **b
                    } else {
                        false
                    }
                }) {
                    if min == &10 {
                        Ok(Self::RoyalFlush)
                    } else {
                        Ok(Self::StraightFlush(*min, *max))
                    }
                } else {
                    Ok(Self::Flush(*max))
                }
            }
            [PokerCards(_, ra), PokerCards(_, rb), PokerCards(_, rc), PokerCards(_, rd), PokerCards(_, re)] =>
            {
                let mut ranks = vec![ra, rb, rc, rd, re];
                ranks.sort();

                Ok(match ranks.as_slice() {
                    [a, b, c, d, e] | [e, a, b, c, d] if a == b && a == c && a == d => {
                        Self::FourOfAKind(**a, **e)
                    }
                    [a, b, c, d, e] | [d, e, a, b, c] if a == b && a == c && d == e => {
                        Self::FullHouse(**a, **d)
                    }
                    [a, b, c, d, e]
                        if *a + 1 == **b && *b + 1 == **c && *c + 1 == **d && *d + 1 == **e =>
                    {
                        Self::Straight(**a, **e)
                    }
                    [a, b, c, _, max] | [_, a, b, c, max] | [_, max, a, b, c]
                        if a == b && a == c =>
                    {
                        Self::ThreeOfAKind(**a, **max)
                    }
                    [a, b, c, d, max] | [a, b, max, c, d] | [max, a, b, c, d]
                        if a == b && c == d =>
                    {
                        Self::TwoPairs(**a, **c, **max)
                    }
                    [a, b, _, _, max]
                    | [_, a, b, _, max]
                    | [_, _, a, b, max]
                    | [_, _, max, a, b]
                        if a == b =>
                    {
                        Self::OnePair(**a, **max)
                    }
                    [_, _, _, _, max] => Self::HighCard(**max),
                    _ => panic!("Missing combination."),
                })
            }
            _ => Err("Hand had a number of cards different from 5.".into()),
        }
    }
}

pub struct Poker;

impl Poker {
    /// Resolves the result of a game, `true` if Player 1 wins, `false` otherwise.
    ///
    /// A game is represented by a String containing 10 cards, the first five represent Player 1's hand.
    pub fn resolve_game(game: &str) -> Result<bool, String> {
        let split: (Vec<_>, Vec<_>) = game
            .split_whitespace()
            .enumerate()
            .partition(|(i, _)| i < &5);
        let hand1 = PokerHand::new(
            &split
                .0
                .into_iter()
                .map(|(_, card)| PokerCards::try_from(card))
                .collect::<Result<Vec<_>, String>>()?,
        )?;
        let hand2 = PokerHand::new(
            &split
                .1
                .into_iter()
                .map(|(_, card)| PokerCards::try_from(card))
                .collect::<Result<Vec<_>, String>>()?,
        )?;
        Ok(hand1 > hand2)
    }

    /// Resolves the result of a tournament, returning a tuple with how many games Player 1 and Player 2 won.
    pub fn resolve_tournament(tournament: &str) -> Result<(u64, u64), String> {
        Ok(tournament
            .lines()
            .map(Self::resolve_game)
            .collect::<Result<Vec<_>, String>>()?
            .into_iter()
            .fold(
                (0, 0),
                |(p1, p2), cur| if cur { (p1 + 1, p2) } else { (p1, p2 + 1) },
            ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_from_card_test() {
        assert!(PokerCards::try_from("AB").is_err());
        assert_eq!(
            PokerCards::try_from("TC"),
            Ok(PokerCards(PokerCardSuit::Clubs, 10))
        );
        assert_eq!(
            PokerCards::try_from("AD"),
            Ok(PokerCards(PokerCardSuit::Diamond, 14))
        );
        assert_eq!(
            PokerCards::try_from("KS"),
            Ok(PokerCards(PokerCardSuit::Spades, 13))
        );
        assert_eq!(
            PokerCards::try_from("2H"),
            Ok(PokerCards(PokerCardSuit::Heart, 2))
        );
    }

    #[test]
    fn poker_hand_test() {
        assert_eq!(
            PokerHand::new(&vec![
                PokerCards(PokerCardSuit::Heart, 5),
                PokerCards(PokerCardSuit::Clubs, 5),
                PokerCards(PokerCardSuit::Spades, 6),
                PokerCards(PokerCardSuit::Spades, 7),
                PokerCards(PokerCardSuit::Diamond, 13)
            ]),
            Ok(PokerHand::OnePair(5, 13))
        );
        assert_eq!(
            PokerHand::new(&vec![
                PokerCards(PokerCardSuit::Clubs, 2),
                PokerCards(PokerCardSuit::Spades, 3),
                PokerCards(PokerCardSuit::Spades, 8),
                PokerCards(PokerCardSuit::Diamond, 8),
                PokerCards(PokerCardSuit::Diamond, 10)
            ]),
            Ok(PokerHand::OnePair(8, 10))
        );
        assert_eq!(
            PokerHand::new(&vec![
                PokerCards(PokerCardSuit::Diamond, 5),
                PokerCards(PokerCardSuit::Clubs, 8),
                PokerCards(PokerCardSuit::Spades, 9),
                PokerCards(PokerCardSuit::Spades, 11),
                PokerCards(PokerCardSuit::Clubs, 14)
            ]),
            Ok(PokerHand::HighCard(14))
        );
        assert_eq!(
            PokerHand::new(&vec![
                PokerCards(PokerCardSuit::Clubs, 2),
                PokerCards(PokerCardSuit::Clubs, 5),
                PokerCards(PokerCardSuit::Diamond, 7),
                PokerCards(PokerCardSuit::Spades, 8),
                PokerCards(PokerCardSuit::Heart, 12)
            ]),
            Ok(PokerHand::HighCard(12))
        );
        assert_eq!(
            PokerHand::new(&vec![
                PokerCards(PokerCardSuit::Diamond, 2),
                PokerCards(PokerCardSuit::Clubs, 9),
                PokerCards(PokerCardSuit::Spades, 14),
                PokerCards(PokerCardSuit::Heart, 14),
                PokerCards(PokerCardSuit::Clubs, 14)
            ]),
            Ok(PokerHand::ThreeOfAKind(14, 9))
        );
        assert_eq!(
            PokerHand::new(&vec![
                PokerCards(PokerCardSuit::Diamond, 3),
                PokerCards(PokerCardSuit::Diamond, 6),
                PokerCards(PokerCardSuit::Diamond, 7),
                PokerCards(PokerCardSuit::Diamond, 10),
                PokerCards(PokerCardSuit::Diamond, 12)
            ]),
            Ok(PokerHand::Flush(12))
        );
        assert_eq!(
            PokerHand::new(&vec![
                PokerCards(PokerCardSuit::Diamond, 4),
                PokerCards(PokerCardSuit::Spades, 6),
                PokerCards(PokerCardSuit::Heart, 9),
                PokerCards(PokerCardSuit::Heart, 12),
                PokerCards(PokerCardSuit::Clubs, 12)
            ]),
            Ok(PokerHand::OnePair(12, 9))
        );
        assert_eq!(
            PokerHand::new(&vec![
                PokerCards(PokerCardSuit::Diamond, 3),
                PokerCards(PokerCardSuit::Diamond, 6),
                PokerCards(PokerCardSuit::Heart, 7),
                PokerCards(PokerCardSuit::Diamond, 12),
                PokerCards(PokerCardSuit::Spades, 12)
            ]),
            Ok(PokerHand::OnePair(12, 7))
        );
        assert_eq!(
            PokerHand::new(&vec![
                PokerCards(PokerCardSuit::Heart, 2),
                PokerCards(PokerCardSuit::Diamond, 2),
                PokerCards(PokerCardSuit::Clubs, 4),
                PokerCards(PokerCardSuit::Diamond, 4),
                PokerCards(PokerCardSuit::Spades, 4)
            ]),
            Ok(PokerHand::FullHouse(4, 2))
        );
        assert_eq!(
            PokerHand::new(&vec![
                PokerCards(PokerCardSuit::Clubs, 3),
                PokerCards(PokerCardSuit::Diamond, 3),
                PokerCards(PokerCardSuit::Spades, 3),
                PokerCards(PokerCardSuit::Spades, 9),
                PokerCards(PokerCardSuit::Diamond, 9)
            ]),
            Ok(PokerHand::FullHouse(3, 9))
        );
    }

    #[test]
    fn resolve_game_test() {
        assert_eq!(
            Poker::resolve_game("5H 5C 6S 7S KD 2C 3S 8S 8D TD"),
            Ok(false)
        );
        assert_eq!(
            Poker::resolve_game("5D 8C 9S JS AC 2C 5C 7D 8S QH"),
            Ok(true)
        );
        assert_eq!(
            Poker::resolve_game("2D 9C AS AH AC 3D 6D 7D TD QD"),
            Ok(false)
        );
        assert_eq!(
            Poker::resolve_game("4D 6S 9H QH QC 3D 6D 7H QD QS"),
            Ok(true)
        );
        assert_eq!(
            Poker::resolve_game("2H 2D 4C 4D 4S 3C 3D 3S 9S 9D"),
            Ok(true)
        );
    }

    #[test]
    fn resolve_tournament_test() {
        let tournament = "5H 5C 6S 7S KD 2C 3S 8S 8D TD
        5D 8C 9S JS AC 2C 5C 7D 8S QH
        2D 9C AS AH AC 3D 6D 7D TD QD
        4D 6S 9H QH QC 3D 6D 7H QD QS
        2H 2D 4C 4D 4S 3C 3D 3S 9S 9D";
        assert_eq!(Poker::resolve_tournament(tournament), Ok((3, 2)));
    }
}
