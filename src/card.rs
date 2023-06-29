use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::cmp::Ordering;

#[derive(Debug, Clone, EnumIter, PartialEq, Eq)]
pub enum CardSuite {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

impl CardSuite {
    fn name(&self) -> String {
        match self {
            CardSuite::Hearts   => String::from("\u{2665}"),
            CardSuite::Spades   => String::from("\u{2660}"),
            CardSuite::Diamonds => String::from("\u{2666}"),
            CardSuite::Clubs    => String::from("\u{2663}"),
        }
    }
}

#[derive(Debug, Copy, Clone, EnumIter, PartialOrd, Ord, PartialEq, Eq)]
pub enum CardRank {
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

impl CardRank {
    fn index(&self) -> usize {
        *self as usize
    }

    fn name(&self) -> String {
        match self {
            CardRank::Two   => String::from("2"),
            CardRank::Three => String::from("3"),
            CardRank::Four  => String::from("4"),
            CardRank::Five  => String::from("5"),
            CardRank::Six  => String::from("6"),
            CardRank::Seven  => String::from("7"),
            CardRank::Eight  => String::from("8"),
            CardRank::Nine  => String::from("9"),
            CardRank::Ten  => String::from("10"),
            CardRank::Jack => String::from("J"),
            CardRank::Queen => String::from("Q"),
            CardRank::King  => String::from("K"),
            CardRank::Ace   => String::from("A"),
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum HandRank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

impl HandRank {
    pub fn name(&self) -> String {
        match self {
            HandRank::StraightFlush => String::from("Straight Flush"),
            HandRank::FourOfAKind => String::from("Four of a Kind"),
            HandRank::FullHouse => String::from("Full House"),
            HandRank::Flush => String::from("Flush"),
            HandRank::Straight => String::from("Straight"),
            HandRank::ThreeOfAKind => String::from("Three of a Kind"),
            HandRank::TwoPair => String::from("Two Pair"),
            HandRank::Pair => String::from("Pair"),
            HandRank::HighCard => String::from("High Card"),
        }
    }
}

#[derive(Clone, Eq)]
pub struct Card {
    pub rank: CardRank,
    pub suite: CardSuite,
}

impl Card {
    pub fn name(&self) -> String {
        self.rank.name() + &self.suite.name()
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut v : Vec<Card> = Vec::new();
        for s in CardSuite::iter() {
            for r in CardRank::iter() {
                v.push( Card {
                    rank : r,
                    suite : s.clone(),
                    } );
            }
        }
        Deck { cards: v }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn count(&self) -> usize {
        self.cards.len()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: Vec::<Card>::new() }
    }

    pub fn new_from_cards(c1: Card, c2: Card, c3: Card, c4: Card, c5: Card) -> Hand {
        let mut hand = Hand { cards: Vec::<Card>::new() };
        hand.add(c1);
        hand.add(c2);
        hand.add(c3);
        hand.add(c4);
        hand.add(c5);
        hand
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card)
    }

    pub fn count(&self) -> usize {
        self.cards.len()
    }

    pub fn card(&self, idx: usize) -> Card {
        self.cards[idx].clone()
    }

    pub fn remove(&mut self, idx: usize) -> Card {
        self.cards.remove(idx)
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        let mut i = 0;
        while i < self.count() {
            s.push_str( &self.cards[i].name() );
            if i+1 < self.count() {
                s.push_str( " " );
            }
            i = i+1;
        }
        s
    }
}

pub fn compare_sorted_hands( h1: &Hand, h2: &Hand ) -> Ordering {
    assert!( h1.count() == h2.count() );
    let mut res = Ordering::Equal;

    let mut i = 0;
    while i < h1.count() {
        res = h1.card(i).cmp(&h2.card(i));
        if res != Ordering::Equal {
            break;
        }
        i += 1;
    }
    res
}

#[derive(Eq)]
pub struct HandWithData {
    hand: Hand,
    kicker: Hand,
    rank: HandRank,
}

impl HandWithData {
    pub fn from_hand(h: &Hand) -> HandWithData {
        assert!(h.count() == 5);
        let mut s = h.clone();
        s.cards.sort_by(|a, b| b.rank.index().cmp(&a.rank.index()));
        // is there a flush?
        let f = 
            s.card(0).suite == s.card(1).suite &&
            s.card(0).suite == s.card(2).suite &&
            s.card(0).suite == s.card(3).suite &&
            s.card(0).suite == s.card(4).suite;
        // is there a straight?
        let mut t =
            s.card(0).rank.index() == s.card(1).rank.index() + 1 &&
            s.card(1).rank.index() == s.card(2).rank.index() + 1 &&
            s.card(2).rank.index() == s.card(3).rank.index() + 1 &&
            s.card(3).rank.index() == s.card(4).rank.index() + 1;
        t = t || (   // wheel
            s.card(0).rank == CardRank::Ace &&
            s.card(1).rank == CardRank::Five &&
            s.card(2).rank == CardRank::Four &&
            s.card(3).rank == CardRank::Three &&
            s.card(4).rank == CardRank::Two );
        // accumulate kicker cards as appropriate
        let mut k = Hand::new();
        // compute rank
        let mut r = HandRank::HighCard;

        if t {
            // for straights, kicker is the high card
            if s.card(0).rank == CardRank::Ace && s.card(1).rank == CardRank::Five {
                k.add( s.card(1) );
            } else {
                k.add( s.card(0) );
            }
        }
        if f && t {
            r = HandRank::StraightFlush;
        }
        if r == HandRank::HighCard {
            if s.card(0) == s.card(1) && s.card(0) == s.card(2) && s.card(0) == s.card(3) {
                r = HandRank::FourOfAKind;
                k.add( s.remove(4) );
            } else if s.card(1) == s.card(2) && s.card(1) == s.card(3) && s.card(1) == s.card(4) {
                r = HandRank::FourOfAKind;
                k.add( s.remove(0) );
            }
        }
        if r == HandRank::HighCard {
            if (s.card(0) == s.card(1) && s.card(0) == s.card(2)) &&
               (s.card(3) == s.card(4)) {
               r = HandRank::FullHouse;
               k.add( s.remove(3) );
               k.add( s.remove(3) );
            } else if (s.card(0) == s.card(1)) &&
               (s.card(2) == s.card(3) && s.card(2) == s.card(4)) {
               r = HandRank::FullHouse;
               k.add( s.remove(0) );
               k.add( s.remove(0) );
            }
        }
        if r == HandRank::HighCard {
            if f {
                r = HandRank::Flush;
            }
        }
        if r == HandRank::HighCard {
            if t {
                r = HandRank::Straight;
            }
        }
        if r == HandRank::HighCard {
            if s.card(0) == s.card(1) && s.card(0) == s.card(2) {
                r = HandRank::ThreeOfAKind;
                k.add( s.remove(3) );
                k.add( s.remove(3) );
            } else if s.card(1) == s.card(2) && s.card(1) == s.card(3) {
                r = HandRank::ThreeOfAKind;
                k.add( s.remove(0) );
                k.add( s.remove(3) );
            } else if s.card(2) == s.card(3) && s.card(2) == s.card(4) {
                r = HandRank::ThreeOfAKind;
                k.add( s.remove(0) );
                k.add( s.remove(0) );
            }
        }
        if r == HandRank::HighCard {
            if s.card(0) == s.card(1) && s.card(2) == s.card(3) {
                r = HandRank::TwoPair;
                k.add( s.remove(4) );
            } else if s.card(0) == s.card(1) && s.card(3) == s.card(4) {
                r = HandRank::TwoPair;
                k.add( s.remove(2) );
            } else if s.card(1) == s.card(2) && s.card(3) == s.card(4) {
                r = HandRank::TwoPair;
                k.add( s.remove(0) );
            }
        }
        if r == HandRank::HighCard {
            if s.card(0) == s.card(1) {
                r = HandRank::Pair;
                k.add( s.remove(2) );
                k.add( s.remove(2) );
                k.add( s.remove(2) );
            } else if s.card(1) == s.card(2) {
                r = HandRank::Pair;
                k.add( s.remove(0) );
                k.add( s.remove(2) );
                k.add( s.remove(2) );
            } else if s.card(2) == s.card(3) {
                r = HandRank::Pair;
                k.add( s.remove(0) );
                k.add( s.remove(0) );
                k.add( s.remove(2) );
            } else if s.card(3) == s.card(4) {
                r = HandRank::Pair;
                k.add( s.remove(0) );
                k.add( s.remove(0) );
                k.add( s.remove(0) );
            }
        }
        if r == HandRank::HighCard {
            k = s;
            s = Hand::new();
        }

        HandWithData{ hand: s, kicker: k, rank: r }
    }

    pub fn to_string(&self) -> String {
        let details = 
            match self.rank {
                HandRank::StraightFlush =>
                    format!("High: {}", self.kicker.card(0).name()),
                HandRank::FourOfAKind =>
                    format!("Rank: {}", self.hand.card(0).name()),
                HandRank::FullHouse =>
                    format!("Rank: {}, Kicker: {}", self.hand.card(0).name(), self.kicker.card(0).name()),
                HandRank::Flush =>
                    format!("Rank: {}", self.hand.to_string()),
                HandRank::Straight => 
                    format!("High: {}", self.kicker.card(0).name()),
                HandRank::ThreeOfAKind =>
                    format!("Rank: {}, Kickers: {}", self.hand.card(0).name(), self.kicker.to_string()),
                HandRank::TwoPair =>
                    format!("High Rank: {}, Low Rank: {}, Kicker: {}", self.hand.card(0).name(), self.hand.card(2).name(), self.kicker.to_string()),
                HandRank::Pair =>
                    format!("Rank: {}, Kickers: {}", self.hand.card(0).name(), self.kicker.to_string()),
                HandRank::HighCard =>
                    format!("Kickers: {}", self.kicker.to_string()),
            };
        format!(" {}, {}", self.rank.name(), details)
    }
}

impl PartialOrd for HandWithData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandWithData {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut res = self.rank.cmp(&other.rank);
        if res == Ordering::Equal {
            res = match self.rank {
                HandRank::StraightFlush =>
                    self.kicker.card(0).cmp(&other.kicker.card(0)),
                HandRank::FourOfAKind =>
                    self.hand.card(0).cmp(&other.hand.card(0)),
                HandRank::FullHouse => {
                    let mut r = self.hand.card(0).cmp(&other.hand.card(0));
                    if r == Ordering::Equal {
                        r = self.kicker.card(0).cmp(&other.kicker.card(0));
                    }
                    r
                }
                HandRank::Flush =>
                    compare_sorted_hands( &self.hand, &other.hand ),
                HandRank::Straight =>
                    self.kicker.card(0).cmp(&other.kicker.card(0)),
                HandRank::ThreeOfAKind => {
                    let mut r = self.hand.card(0).cmp(&other.hand.card(0));
                    if r == Ordering::Equal {
                        r = compare_sorted_hands( &self.kicker, &other.kicker );
                    }
                    r
                }
                HandRank::TwoPair => {
                    let mut r = self.hand.card(0).cmp(&other.hand.card(0));
                    if r == Ordering::Equal {
                        r = self.hand.card(2).cmp(&other.hand.card(2));
                        if r == Ordering::Equal {
                            r = self.kicker.card(0).cmp(&other.kicker.card(0));
                        }
                    }
                    r
                }
                HandRank::Pair => {
                    let mut r = self.hand.card(0).cmp(&other.hand.card(0));
                    if r == Ordering::Equal {
                        r = compare_sorted_hands( &self.kicker, &other.kicker );
                    }
                    r
                }
                HandRank::HighCard =>
                    compare_sorted_hands( &self.kicker, &other.kicker ),
            };
        }
        res
    }
}

impl PartialEq for HandWithData {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

// Form all 5 card candidates
fn form_all_candidates(h1: Hand, h2: Hand) -> Vec::<Hand> {
    assert_eq!( h1.count(), 2 );
    assert_eq!( h2.count(), 5 );

    let mut hands = Vec::<Hand>::new();
    // 0 own cards
    hands.push(
        h2.clone()
    );
    // 1 own cards
    hands.push( 
        Hand::new_from_cards( h1.card(0), h2.card(0), h2.card(1), h2.card(2), h2.card(3) )
    );
    hands.push( 
        Hand::new_from_cards( h1.card(0), h2.card(0), h2.card(1), h2.card(2), h2.card(4) )
    );
    hands.push( 
        Hand::new_from_cards( h1.card(0), h2.card(0), h2.card(1), h2.card(3), h2.card(4) )
    );
    hands.push( 
        Hand::new_from_cards( h1.card(0), h2.card(0), h2.card(2), h2.card(3), h2.card(4) )
    );
    hands.push( 
        Hand::new_from_cards( h1.card(0), h2.card(1), h2.card(2), h2.card(3), h2.card(4) )
    );
    hands.push( 
        Hand::new_from_cards( h1.card(1), h2.card(0), h2.card(1), h2.card(2), h2.card(3) )
    );
    hands.push( 
        Hand::new_from_cards( h1.card(1), h2.card(0), h2.card(1), h2.card(2), h2.card(4) )
    );
    hands.push( 
        Hand::new_from_cards( h1.card(1), h2.card(0), h2.card(1), h2.card(3), h2.card(4) )
    );
    hands.push( 
        Hand::new_from_cards( h1.card(1), h2.card(0), h2.card(2), h2.card(3), h2.card(4) )
    );
    hands.push( 
        Hand::new_from_cards( h1.card(1), h2.card(1), h2.card(2), h2.card(3), h2.card(4) )
    );
    // 2 own cards
    hands.push( 
        Hand::new_from_cards( h1.card(0), h1.card(1), h2.card(0), h2.card(1), h2.card(2) )
    );
    hands.push( 
        Hand::new_from_cards( h1.card(0), h1.card(1), h2.card(0), h2.card(1), h2.card(3) )
    );
    hands.push( 
        Hand::new_from_cards( h1.card(0), h1.card(1), h2.card(0), h2.card(1), h2.card(4) )
    );
    hands.push( 
        Hand::new_from_cards( h1.card(0), h1.card(1), h2.card(0), h2.card(2), h2.card(3) )
    );
    hands.push( 
        Hand::new_from_cards( h1.card(0), h1.card(1), h2.card(0), h2.card(2), h2.card(4) )
    );
    hands.push( 
        Hand::new_from_cards( h1.card(0), h1.card(1), h2.card(0), h2.card(3), h2.card(4) )
    );
    hands.push( 
        Hand::new_from_cards( h1.card(0), h1.card(1), h2.card(1), h2.card(2), h2.card(3) )
    );
    hands.push( 
        Hand::new_from_cards( h1.card(0), h1.card(1), h2.card(1), h2.card(2), h2.card(4) )
    );
    hands.push( 
        Hand::new_from_cards( h1.card(0), h1.card(1), h2.card(1), h2.card(3), h2.card(4) )
    );
    hands.push( 
        Hand::new_from_cards( h1.card(0), h1.card(1), h2.card(2), h2.card(3), h2.card(4) )
    );

    hands
}

pub fn find_best_hand(h1: Hand, h2: Hand, verbose: bool) -> HandWithData {
    let hands = form_all_candidates(h1, h2);
    let mut best = HandWithData::from_hand(&hands[0]);

    if verbose {
        println!("Candidates:");
    }
    for h in hands {
        let hwd = HandWithData::from_hand(&h);
        if verbose {
            println!(" {} -> {}", h.to_string(), hwd.to_string());
        }
        if hwd > best {
            best = hwd;
        }
    }
    best
}

#[cfg(test)]
mod tests;

