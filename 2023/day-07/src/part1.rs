use crate::{error::Error, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    #[tracing::instrument]
    fn from_str(input: char) -> Result<Self> {
        match input {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err(Error::CouldNotParseCard(input.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    #[tracing::instrument]
    fn from_cards(cards: &[Card]) -> Result<Self> {
        if cards.len() != 5 {
            return Err(Error::UnexpectedNumberOfCards);
        }

        let mut cards = cards.to_vec();
        cards.sort();

        if cards[0] == cards[4] {
            return Ok(HandType::FiveOfAKind);
        }

        if cards[0] == cards[3] || cards[1] == cards[4] {
            return Ok(HandType::FourOfAKind);
        }

        if (cards[0] == cards[2] && cards[3] == cards[4])
            || (cards[0] == cards[1] && cards[2] == cards[4])
        {
            return Ok(HandType::FullHouse);
        }

        if cards[0] == cards[2] || cards[1] == cards[3] || cards[2] == cards[4] {
            return Ok(HandType::ThreeOfAKind);
        }

        if (cards[0] == cards[1] && cards[2] == cards[3])
            || (cards[0] == cards[1] && cards[3] == cards[4])
            || (cards[1] == cards[2] && cards[3] == cards[4])
        {
            return Ok(HandType::TwoPair);
        }

        if cards[0] == cards[1]
            || cards[1] == cards[2]
            || cards[2] == cards[3]
            || cards[3] == cards[4]
        {
            return Ok(HandType::OnePair);
        }

        Ok(HandType::HighCard)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
}

impl Hand {
    #[tracing::instrument]
    fn new(cards: [Card; 5]) -> Result<Self> {
        let hand_type = HandType::from_cards(&cards)?;
        
        Ok(Self { cards, hand_type })
    }

    #[tracing::instrument]
    fn from_str(input: &str) -> Result<Self> {
        let mut cards = [Card::Two; 5];
        for (i, card) in input.chars().enumerate() {
            cards[i] = Card::from_str(card)?;
        }
        Self::new(cards)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => {
                for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    match self_card.cmp(other_card) {
                        std::cmp::Ordering::Equal => continue,
                        other => return other,
                    }
                }
                std::cmp::Ordering::Equal
            }
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[tracing::instrument]
fn order_hands(hands: &[Hand]) -> Vec<Hand> {
    let mut hands = hands.to_vec();
    hands.sort();
    hands
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HandAndBet {
    hand: Hand,
    bet: u32,
}

impl HandAndBet {
    #[tracing::instrument]
    fn from_str(input: &str) -> Result<Self> {
        let mut split = input.split_whitespace();

        let hand = split
            .next()
            .ok_or_else(|| Error::CouldNotParseHandAndBet(input.to_string()))?;

        let hand = Hand::from_str(hand)?;

        let bet = split
            .next()
            .ok_or_else(|| Error::CouldNotParseHandAndBet(input.to_string()))?
            .parse::<u32>()
            .map_err(Error::CouldNotParseNumber)?;

        Ok(Self { hand, bet })
    }
}

impl Ord for HandAndBet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for HandAndBet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[tracing::instrument]
fn sort_hands_and_bets(hands_and_bets: &[HandAndBet]) -> Vec<HandAndBet> {
    let mut hands_and_bets = hands_and_bets.to_vec();
    hands_and_bets.sort();
    hands_and_bets
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32> {
    let bets_and_hands = input
        .lines()
        .map(|line| HandAndBet::from_str(line.trim()))
        .collect::<Result<Vec<HandAndBet>>>()?;

    let ordered_hands_and_bets = sort_hands_and_bets(&bets_and_hands);

    let total_winnings = ordered_hands_and_bets
        .iter()
        .enumerate()
        .map(|(i, hand_and_bet)| hand_and_bet.bet * (i + 1) as u32)
        .sum::<u32>();

    Ok(total_winnings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_parse_cards_correctly() -> miette::Result<()> {
        assert_eq!(Card::Ace, Card::from_str('A')?);
        assert_eq!(Card::King, Card::from_str('K')?);
        assert_eq!(Card::Queen, Card::from_str('Q')?);
        assert_eq!(Card::Jack, Card::from_str('J')?);
        assert_eq!(Card::Ten, Card::from_str('T')?);
        assert_eq!(Card::Nine, Card::from_str('9')?);
        assert_eq!(Card::Eight, Card::from_str('8')?);
        assert_eq!(Card::Seven, Card::from_str('7')?);
        assert_eq!(Card::Six, Card::from_str('6')?);
        assert_eq!(Card::Five, Card::from_str('5')?);
        assert_eq!(Card::Four, Card::from_str('4')?);
        assert_eq!(Card::Three, Card::from_str('3')?);
        assert_eq!(Card::Two, Card::from_str('2')?);

        Ok(())
    }

    #[test]
    fn it_should_ord_cards_correctly() -> miette::Result<()> {
        assert_eq!(true, Card::Ace > Card::King);
        assert_eq!(true, Card::Two < Card::Three);
        assert_eq!(true, Card::Two == Card::Two);

        Ok(())
    }

    #[test]
    fn it_should_make_right_hand_type() -> miette::Result<()> {
        assert_eq!(
            HandType::FiveOfAKind,
            HandType::from_cards(&[Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace])?
        );

        assert_eq!(
            HandType::FourOfAKind,
            HandType::from_cards(&[Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Eight])?
        );

        assert_eq!(
            HandType::FourOfAKind,
            HandType::from_cards(&[Card::Ace, Card::Ace, Card::Eight, Card::Ace, Card::Ace])?
        );

        assert_eq!(
            HandType::FullHouse,
            HandType::from_cards(&[Card::Ace, Card::Ace, Card::Eight, Card::Eight, Card::Ace])?
        );

        assert_eq!(
            HandType::ThreeOfAKind,
            HandType::from_cards(&[Card::King, Card::Eight, Card::Eight, Card::Eight, Card::Ace])?
        );

        assert_eq!(
            HandType::ThreeOfAKind,
            HandType::from_cards(&[Card::Ace, Card::Eight, Card::Eight, Card::Eight, Card::King])?
        );

        assert_eq!(
            HandType::TwoPair,
            HandType::from_cards(&[Card::Ace, Card::Ace, Card::Eight, Card::Eight, Card::King])?
        );

        assert_eq!(
            HandType::TwoPair,
            HandType::from_cards(&[Card::Ace, Card::Ace, Card::Eight, Card::King, Card::King])?
        );

        assert_eq!(
            HandType::TwoPair,
            HandType::from_cards(&[Card::Ace, Card::King, Card::Eight, Card::King, Card::Eight])?
        );

        assert_eq!(
            HandType::OnePair,
            HandType::from_cards(&[Card::Ace, Card::King, Card::Ace, Card::Four, Card::Eight])?
        );

        assert_eq!(
            HandType::OnePair,
            HandType::from_cards(&[Card::Ace, Card::King, Card::King, Card::Four, Card::Eight])?
        );

        assert_eq!(
            HandType::OnePair,
            HandType::from_cards(&[Card::Ace, Card::King, Card::Eight, Card::Four, Card::Eight])?
        );

        assert_eq!(
            HandType::OnePair,
            HandType::from_cards(&[Card::Ace, Card::King, Card::Four, Card::Four, Card::Eight])?
        );

        Ok(())
    }

    #[test]
    fn it_should_rank_hands_correctly() -> miette::Result<()> {
        assert_eq!(
            true,
            Hand::new([Card::King, Card::King, Card::King, Card::King, Card::King])?
                > Hand::new([Card::Two, Card::Two, Card::Ace, Card::Ace, Card::Ace])?
        );

        assert_eq!(
            true,
            Hand::new([
                Card::Three,
                Card::Three,
                Card::Three,
                Card::Three,
                Card::Two
            ])? > Hand::new([Card::Two, Card::Ace, Card::Ace, Card::Ace, Card::Ace])?
        );

        Ok(())
    }

    #[test]
    fn it_should_order_hands_correctly() -> miette::Result<()> {
        let hand1 = Hand::from_str("32T3K")?;
        let hand2 = Hand::from_str("T55J5")?;
        let hand3 = Hand::from_str("KK677")?;
        let hand4 = Hand::from_str("KTJJT")?;
        let hand5 = Hand::from_str("QQQJA")?;

        let hands = vec![hand1, hand2, hand3, hand4, hand5];

        let ordered_hands = order_hands(&hands);

        assert_eq!(ordered_hands[0], Hand::from_str("32T3K")?);
        assert_eq!(ordered_hands[1], Hand::from_str("KTJJT")?);
        assert_eq!(ordered_hands[2], Hand::from_str("KK677")?);
        assert_eq!(ordered_hands[3], Hand::from_str("T55J5")?);
        assert_eq!(ordered_hands[4], Hand::from_str("QQQJA")?);

        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";
        assert_eq!(6440, process(input)?);
        Ok(())
    }
}
