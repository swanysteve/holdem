#[cfg(test)]
mod tests {
    //use super::*;
    use crate::card::*;

    #[test]
    fn cardrank_sanity() {
        assert!( CardRank::Seven.index() < CardRank::Eight.index() );
        assert!( CardRank::Seven.index() == CardRank::Seven.index() );
        assert!( CardRank::Ace.index() > CardRank::Two.index() );
        assert!( CardRank::Seven < CardRank::Eight );
        assert!( CardRank::Seven == CardRank::Seven );
        assert!( CardRank::Ace > CardRank::Two );
    }

    #[test]
    fn card_sanity() {
        let c1 = Card{ rank: CardRank::Seven, suite: CardSuite::Clubs };
        let c2 = Card{ rank: CardRank::Eight, suite: CardSuite::Clubs };
        let c3 = Card{ rank: CardRank::Seven, suite: CardSuite::Diamonds };

        assert!( c1 != c2 );
        assert!( c2 != c3 );
        assert!( c1 == c3 );

        assert_eq!( c1.name(), String::from("7♣") );
    }

    #[test]
    fn deck_sanity() {
        let mut d = Deck::new();
        assert_eq!( d.count(), 52 );
        let _c = d.draw();
        assert_eq!( d.count(), 51 );
        let mut s = Deck::new();
        s.shuffle();
        assert_eq!( s.count(), 52 );
    }

    #[test]
    fn hand_sanity() {
        let mut h = Hand::new();
        h.add( Card{ rank: CardRank::Seven, suite: CardSuite::Clubs } );
        assert_eq!( h.count(), 1 );
    }

    #[test]
    fn compare_sorted_hands_sanity() {
        let h1 = Hand::new_from_cards(
            Card{ rank: CardRank::Ten, suite: CardSuite::Spades },
            Card{ rank: CardRank::Nine, suite: CardSuite::Spades },
            Card{ rank: CardRank::Eight, suite: CardSuite::Spades },
            Card{ rank: CardRank::Seven, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Six, suite: CardSuite::Clubs } );
        let h2 = Hand::new_from_cards(
            Card{ rank: CardRank::Jack, suite: CardSuite::Spades },
            Card{ rank: CardRank::Nine, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Eight, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Seven, suite: CardSuite::Spades },
            Card{ rank: CardRank::Six, suite: CardSuite::Spades } );
        assert!( compare_sorted_hands( &h1, &h2) == Ordering::Less );
        assert!( compare_sorted_hands( &h2, &h1) == Ordering::Greater );
        assert!( compare_sorted_hands( &h2, &h2) == Ordering::Equal );

        // need not have 5 cards
        let mut h3 = Hand::new();
        h3.add( Card{ rank: CardRank::Seven, suite: CardSuite::Clubs } );
        h3.add( Card{ rank: CardRank::Two, suite: CardSuite::Clubs } );
        let mut h4 = Hand::new();
        h4.add( Card{ rank: CardRank::Seven, suite: CardSuite::Hearts } );
        h4.add( Card{ rank: CardRank::Two, suite: CardSuite::Hearts } );
        let mut h5 = Hand::new();
        h5.add( Card{ rank: CardRank::Seven, suite: CardSuite::Spades } );
        h5.add( Card{ rank: CardRank::Three, suite: CardSuite::Spades } );
        assert!( compare_sorted_hands( &h3, &h4) == Ordering::Equal );
        assert!( compare_sorted_hands( &h3, &h5) == Ordering::Less );
        assert!( compare_sorted_hands( &h4, &h5) == Ordering::Less );
    }

    #[test]
    fn handwithdata_sanity() {
        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Two, suite: CardSuite::Spades },
            Card{ rank: CardRank::Three, suite: CardSuite::Spades },
            Card{ rank: CardRank::Four, suite: CardSuite::Spades },
            Card{ rank: CardRank::Five, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Six, suite: CardSuite::Clubs } );
        let hwd1 = HandWithData::from_hand( &h );
        assert!( hwd1.rank == HandRank::Straight );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Two, suite: CardSuite::Spades },
            Card{ rank: CardRank::Three, suite: CardSuite::Spades },
            Card{ rank: CardRank::Four, suite: CardSuite::Spades },
            Card{ rank: CardRank::Five, suite: CardSuite::Spades },
            Card{ rank: CardRank::Eight, suite: CardSuite::Spades } );
        let hwd2 = HandWithData::from_hand( &h );
        assert!( hwd2.rank == HandRank::Flush );

        assert!( hwd2 > hwd1 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Ace, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Two, suite: CardSuite::Spades },
            Card{ rank: CardRank::Three, suite: CardSuite::Spades },
            Card{ rank: CardRank::Four, suite: CardSuite::Spades },
            Card{ rank: CardRank::Five, suite: CardSuite::Clubs });
        let hwd3 = HandWithData::from_hand( &h );
        assert!( hwd3.rank == HandRank::Straight );

        assert!( hwd2 > hwd3 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Ace, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Ace, suite: CardSuite::Spades },
            Card{ rank: CardRank::Three, suite: CardSuite::Spades },
            Card{ rank: CardRank::Four, suite: CardSuite::Spades },
            Card{ rank: CardRank::Five, suite: CardSuite::Clubs });
        let hwd4 = HandWithData::from_hand( &h );
        assert!( hwd4.rank == HandRank::Pair );

        assert!( hwd4 < hwd3 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Ace, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Ace, suite: CardSuite::Spades },
            Card{ rank: CardRank::Three, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Three, suite: CardSuite::Spades },
            Card{ rank: CardRank::Three, suite: CardSuite::Clubs });
        let hwd5 = HandWithData::from_hand( &h );
        assert!( hwd5.rank == HandRank::FullHouse );

        assert!( hwd5 > hwd1 );
        assert!( hwd5 > hwd2 );
        assert!( hwd5 > hwd3 );
        assert!( hwd5 > hwd4 );
    }

    #[test]
    fn straightflush_sanity() {
        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Two, suite: CardSuite::Spades },
            Card{ rank: CardRank::Three, suite: CardSuite::Spades },
            Card{ rank: CardRank::Four, suite: CardSuite::Spades },
            Card{ rank: CardRank::Five, suite: CardSuite::Spades },
            Card{ rank: CardRank::Six, suite: CardSuite::Spades } );
        let hwd1 = HandWithData::from_hand( &h );
        assert!( hwd1.rank == HandRank::StraightFlush );
        assert!( hwd1.kicker.count() == 1 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Ace, suite: CardSuite::Spades },
            Card{ rank: CardRank::Two, suite: CardSuite::Spades },
            Card{ rank: CardRank::Three, suite: CardSuite::Spades },
            Card{ rank: CardRank::Four, suite: CardSuite::Spades },
            Card{ rank: CardRank::Five, suite: CardSuite::Spades } );
        let hwd2 = HandWithData::from_hand( &h );
        assert!( hwd2.rank == HandRank::StraightFlush );
        assert!( hwd2.kicker.count() == 1 );

        assert!( hwd1 > hwd2 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Ace, suite: CardSuite::Spades },
            Card{ rank: CardRank::King, suite: CardSuite::Spades },
            Card{ rank: CardRank::Queen, suite: CardSuite::Spades },
            Card{ rank: CardRank::Jack, suite: CardSuite::Spades },
            Card{ rank: CardRank::Ten, suite: CardSuite::Spades } );
        let hwd3 = HandWithData::from_hand( &h );
        assert!( hwd3.rank == HandRank::StraightFlush );
        assert!( hwd3.kicker.count() == 1 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Ace, suite: CardSuite::Hearts },
            Card{ rank: CardRank::King, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Queen, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Jack, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Ten, suite: CardSuite::Hearts } );
        let hwd4 = HandWithData::from_hand( &h );
        assert!( hwd4.rank == HandRank::StraightFlush );
        assert!( hwd4.kicker.count() == 1 );
    }

    #[test]
    fn fourofakind_sanity() {
        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Two, suite: CardSuite::Spades },
            Card{ rank: CardRank::Two, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Two, suite: CardSuite::Diamonds },
            Card{ rank: CardRank::Two, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Six, suite: CardSuite::Spades } );
        let hwd1 = HandWithData::from_hand( &h );
        assert!( hwd1.rank == HandRank::FourOfAKind );
        assert!( hwd1.kicker.count() == 1 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Two, suite: CardSuite::Spades },
            Card{ rank: CardRank::Six, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Six, suite: CardSuite::Diamonds },
            Card{ rank: CardRank::Six, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Six, suite: CardSuite::Spades } );
        let hwd2 = HandWithData::from_hand( &h );
        assert!( hwd2.rank == HandRank::FourOfAKind );
        assert!( hwd2.kicker.count() == 1 );

        assert!( hwd1 < hwd2 );
    }

    #[test]
    fn fullhouse_sanity() {
        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Two, suite: CardSuite::Spades },
            Card{ rank: CardRank::Two, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Two, suite: CardSuite::Diamonds },
            Card{ rank: CardRank::Six, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Six, suite: CardSuite::Spades } );
        let hwd1 = HandWithData::from_hand( &h );
        assert!( hwd1.rank == HandRank::FullHouse );
        assert!( hwd1.kicker.count() == 2 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Ten, suite: CardSuite::Spades },
            Card{ rank: CardRank::Ten, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Ten, suite: CardSuite::Diamonds },
            Card{ rank: CardRank::Six, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Six, suite: CardSuite::Spades } );
        let hwd2 = HandWithData::from_hand( &h );
        assert!( hwd2.rank == HandRank::FullHouse );
        assert!( hwd2.kicker.count() == 2 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Ten, suite: CardSuite::Spades },
            Card{ rank: CardRank::Ten, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Ten, suite: CardSuite::Diamonds },
            Card{ rank: CardRank::King, suite: CardSuite::Hearts },
            Card{ rank: CardRank::King, suite: CardSuite::Spades } );
        let hwd3 = HandWithData::from_hand( &h );
        assert!( hwd3.rank == HandRank::FullHouse );
        assert!( hwd3.kicker.count() == 2 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Jack, suite: CardSuite::Spades },
            Card{ rank: CardRank::Jack, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Jack, suite: CardSuite::Diamonds },
            Card{ rank: CardRank::Six, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Six, suite: CardSuite::Spades } );
        let hwd4 = HandWithData::from_hand( &h );
        assert!( hwd4.rank == HandRank::FullHouse );

        assert!( hwd1 < hwd2 );
        assert!( hwd1 < hwd3 );
        assert!( hwd1 < hwd4 );
        assert!( hwd2 < hwd3 );
        assert!( hwd2 < hwd4 );
        assert!( hwd3 < hwd4 );
    }

    #[test]
    fn flush_sanity() {
        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Two, suite: CardSuite::Spades },
            Card{ rank: CardRank::Three, suite: CardSuite::Spades },
            Card{ rank: CardRank::Four, suite: CardSuite::Spades },
            Card{ rank: CardRank::Five, suite: CardSuite::Spades },
            Card{ rank: CardRank::Seven, suite: CardSuite::Spades } );
        let hwd1 = HandWithData::from_hand( &h );
        assert!( hwd1.rank == HandRank::Flush );
        assert!( hwd1.kicker.count() == 0 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Two, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Three, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Four, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Five, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Seven, suite: CardSuite::Clubs } );
        let hwd2 = HandWithData::from_hand( &h );
        assert!( hwd2.rank == HandRank::Flush );
        assert!( hwd2.kicker.count() == 0 );

        assert!( hwd1 == hwd2 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Two, suite: CardSuite::Diamonds },
            Card{ rank: CardRank::Three, suite: CardSuite::Diamonds },
            Card{ rank: CardRank::Four, suite: CardSuite::Diamonds },
            Card{ rank: CardRank::Five, suite: CardSuite::Diamonds },
            Card{ rank: CardRank::Eight, suite: CardSuite::Diamonds } );
        let hwd3 = HandWithData::from_hand( &h );
        assert!( hwd3.rank == HandRank::Flush );
        assert!( hwd3.kicker.count() == 0 );

        assert!( hwd1 < hwd3 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Two, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Three, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Four, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Six, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Seven, suite: CardSuite::Hearts } );
        let hwd4 = HandWithData::from_hand( &h );
        assert!( hwd4.rank == HandRank::Flush );
        assert!( hwd4.kicker.count() == 0 );

        assert!( hwd1 < hwd4 );
        assert!( hwd3 > hwd4 );
    }

    #[test]
    fn straight_sanity() {
        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Two, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Three, suite: CardSuite::Spades },
            Card{ rank: CardRank::Four, suite: CardSuite::Spades },
            Card{ rank: CardRank::Five, suite: CardSuite::Spades },
            Card{ rank: CardRank::Six, suite: CardSuite::Spades } );
        let hwd1 = HandWithData::from_hand( &h );
        assert!( hwd1.rank == HandRank::Straight );
        assert!( hwd1.kicker.count() == 1 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Two, suite: CardSuite::Spades },
            Card{ rank: CardRank::Three, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Four, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Five, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Six, suite: CardSuite::Clubs } );
        let hwd2 = HandWithData::from_hand( &h );
        assert!( hwd2.rank == HandRank::Straight );
        assert!( hwd2.kicker.count() == 1 );

        assert!( hwd1 == hwd2 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Three, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Four, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Five, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Six, suite: CardSuite::Spades },
            Card{ rank: CardRank::Seven, suite: CardSuite::Clubs } );
        let hwd3 = HandWithData::from_hand( &h );
        assert!( hwd3.rank == HandRank::Straight );
        assert!( hwd3.kicker.count() == 1 );

        assert!( hwd1 < hwd3 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Two, suite: CardSuite::Spades },
            Card{ rank: CardRank::Three, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Four, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Five, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Ace, suite: CardSuite::Clubs } );
        let hwd4 = HandWithData::from_hand( &h );
        assert!( hwd4.rank == HandRank::Straight );
        assert!( hwd4.kicker.count() == 1 );

        assert!( hwd1 > hwd4 );
    }

    #[test]
    fn threeofakind_sanity() {
        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Six, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Six, suite: CardSuite::Spades },
            Card{ rank: CardRank::Six, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Five, suite: CardSuite::Spades },
            Card{ rank: CardRank::Four, suite: CardSuite::Spades } );
        let hwd1 = HandWithData::from_hand( &h );
        assert!( hwd1.rank == HandRank::ThreeOfAKind );
        assert!( hwd1.kicker.count() == 2 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Six, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Six, suite: CardSuite::Spades },
            Card{ rank: CardRank::Six, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Five, suite: CardSuite::Spades },
            Card{ rank: CardRank::Seven, suite: CardSuite::Spades } );
        let hwd2 = HandWithData::from_hand( &h );
        assert!( hwd2.rank == HandRank::ThreeOfAKind );
        assert!( hwd2.kicker.count() == 2 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Six, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Six, suite: CardSuite::Spades },
            Card{ rank: CardRank::Six, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Eight, suite: CardSuite::Spades },
            Card{ rank: CardRank::Seven, suite: CardSuite::Spades } );
        let hwd3 = HandWithData::from_hand( &h );
        assert!( hwd3.rank == HandRank::ThreeOfAKind );
        assert!( hwd3.kicker.count() == 2 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Seven, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Seven, suite: CardSuite::Spades },
            Card{ rank: CardRank::Seven, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Five, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Eight, suite: CardSuite::Spades } );
        let hwd4 = HandWithData::from_hand( &h );
        assert!( hwd4.rank == HandRank::ThreeOfAKind );
        assert!( hwd4.kicker.count() == 2 );

        assert!( hwd1 < hwd2 );
        assert!( hwd1 < hwd3 );
        assert!( hwd1 < hwd4 );
        assert!( hwd2 < hwd3 );
        assert!( hwd2 < hwd4 );
        assert!( hwd3 < hwd4 );
    }

    #[test]
    fn twopair_sanity() {
        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Six, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Six, suite: CardSuite::Spades },
            Card{ rank: CardRank::Five, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Five, suite: CardSuite::Spades },
            Card{ rank: CardRank::Four, suite: CardSuite::Spades } );
        let hwd1 = HandWithData::from_hand( &h );
        assert!( hwd1.rank == HandRank::TwoPair );
        assert!( hwd1.kicker.count() == 1 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Six, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Six, suite: CardSuite::Spades },
            Card{ rank: CardRank::Five, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Five, suite: CardSuite::Spades },
            Card{ rank: CardRank::Jack, suite: CardSuite::Spades } );
        let hwd2 = HandWithData::from_hand( &h );
        assert!( hwd2.rank == HandRank::TwoPair );
        assert!( hwd2.kicker.count() == 1 );

        assert!( hwd1 < hwd2 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Six, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Six, suite: CardSuite::Spades },
            Card{ rank: CardRank::Four, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Four, suite: CardSuite::Spades },
            Card{ rank: CardRank::Five, suite: CardSuite::Spades } );
        let hwd3 = HandWithData::from_hand( &h );
        assert!( hwd3.rank == HandRank::TwoPair );
        assert!( hwd3.kicker.count() == 1 );

        assert!( hwd1 > hwd3 );
        assert!( hwd2 > hwd3 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Five, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Five, suite: CardSuite::Spades },
            Card{ rank: CardRank::Four, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Four, suite: CardSuite::Spades },
            Card{ rank: CardRank::King, suite: CardSuite::Spades } );
        let hwd4 = HandWithData::from_hand( &h );
        assert!( hwd4.rank == HandRank::TwoPair );
        assert!( hwd4.kicker.count() == 1 );

        assert!( hwd1 > hwd4 );
        assert!( hwd2 > hwd4 );
        assert!( hwd3 > hwd4 );
    }

    #[test]
    fn pair_sanity() {
        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Six, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Six, suite: CardSuite::Spades },
            Card{ rank: CardRank::Five, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Two, suite: CardSuite::Spades },
            Card{ rank: CardRank::Four, suite: CardSuite::Spades } );
        let hwd1 = HandWithData::from_hand( &h );
        assert!( hwd1.rank == HandRank::Pair );
        assert!( hwd1.kicker.count() == 3 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Six, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Six, suite: CardSuite::Spades },
            Card{ rank: CardRank::Five, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Seven, suite: CardSuite::Spades },
            Card{ rank: CardRank::Four, suite: CardSuite::Spades } );
        let hwd2 = HandWithData::from_hand( &h );
        assert!( hwd2.rank == HandRank::Pair );
        assert!( hwd2.kicker.count() == 3 );

        assert!( hwd1 < hwd2 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Six, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Six, suite: CardSuite::Spades },
            Card{ rank: CardRank::Five, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Seven, suite: CardSuite::Spades },
            Card{ rank: CardRank::Eight, suite: CardSuite::Spades } );
        let hwd3 = HandWithData::from_hand( &h );
        assert!( hwd3.rank == HandRank::Pair );
        assert!( hwd3.kicker.count() == 3 );

        assert!( hwd1 < hwd3 );
        assert!( hwd2 < hwd3 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Six, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Six, suite: CardSuite::Spades },
            Card{ rank: CardRank::Nine, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Seven, suite: CardSuite::Spades },
            Card{ rank: CardRank::Eight, suite: CardSuite::Spades } );
        let hwd4 = HandWithData::from_hand( &h );
        assert!( hwd4.rank == HandRank::Pair );
        assert!( hwd4.kicker.count() == 3 );

        assert!( hwd1 < hwd4 );
        assert!( hwd2 < hwd4 );
        assert!( hwd3 < hwd4 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Five, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Five, suite: CardSuite::Spades },
            Card{ rank: CardRank::Nine, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Seven, suite: CardSuite::Spades },
            Card{ rank: CardRank::Eight, suite: CardSuite::Spades } );
        let hwd5 = HandWithData::from_hand( &h );
        assert!( hwd5.rank == HandRank::Pair );
        assert!( hwd5.kicker.count() == 3 );

        assert!( hwd1 > hwd5 );
        assert!( hwd2 > hwd5 );
        assert!( hwd3 > hwd5 );
        assert!( hwd4 > hwd5 );
    }

    #[test]
    fn highcard_sanity() {
        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Six, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Seven, suite: CardSuite::Spades },
            Card{ rank: CardRank::Five, suite: CardSuite::Hearts },
            Card{ rank: CardRank::King, suite: CardSuite::Spades },
            Card{ rank: CardRank::Four, suite: CardSuite::Spades } );
        let hwd1 = HandWithData::from_hand( &h );
        assert!( hwd1.rank == HandRank::HighCard );
        assert!( hwd1.kicker.count() == 5 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Six, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Seven, suite: CardSuite::Spades },
            Card{ rank: CardRank::Five, suite: CardSuite::Hearts },
            Card{ rank: CardRank::Queen, suite: CardSuite::Spades },
            Card{ rank: CardRank::Four, suite: CardSuite::Spades } );
        let hwd2 = HandWithData::from_hand( &h );
        assert!( hwd2.rank == HandRank::HighCard );
        assert!( hwd2.kicker.count() == 5 );

        assert!( hwd1 > hwd2 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Six, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Eight, suite: CardSuite::Spades },
            Card{ rank: CardRank::Five, suite: CardSuite::Hearts },
            Card{ rank: CardRank::King, suite: CardSuite::Spades },
            Card{ rank: CardRank::Four, suite: CardSuite::Spades } );
        let hwd3 = HandWithData::from_hand( &h );
        assert!( hwd3.rank == HandRank::HighCard );
        assert!( hwd3.kicker.count() == 5 );

        assert!( hwd1 < hwd3 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Seven, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Eight, suite: CardSuite::Spades },
            Card{ rank: CardRank::Five, suite: CardSuite::Hearts },
            Card{ rank: CardRank::King, suite: CardSuite::Spades },
            Card{ rank: CardRank::Four, suite: CardSuite::Spades } );
        let hwd4 = HandWithData::from_hand( &h );
        assert!( hwd4.rank == HandRank::HighCard );
        assert!( hwd4.kicker.count() == 5 );

        assert!( hwd3 < hwd4 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Seven, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Eight, suite: CardSuite::Spades },
            Card{ rank: CardRank::Six, suite: CardSuite::Hearts },
            Card{ rank: CardRank::King, suite: CardSuite::Spades },
            Card{ rank: CardRank::Four, suite: CardSuite::Spades } );
        let hwd5 = HandWithData::from_hand( &h );
        assert!( hwd5.rank == HandRank::HighCard );
        assert!( hwd5.kicker.count() == 5 );

        assert!( hwd4 < hwd5 );

        let h = Hand::new_from_cards(
            Card{ rank: CardRank::Seven, suite: CardSuite::Clubs },
            Card{ rank: CardRank::Eight, suite: CardSuite::Spades },
            Card{ rank: CardRank::Six, suite: CardSuite::Hearts },
            Card{ rank: CardRank::King, suite: CardSuite::Spades },
            Card{ rank: CardRank::Five, suite: CardSuite::Spades } );
        let hwd6 = HandWithData::from_hand( &h );
        assert!( hwd6.rank == HandRank::HighCard );
        assert!( hwd6.kicker.count() == 5 );

        assert!( hwd5 < hwd6 );
    }


    #[test]
    fn candidate_sanity() {
        let mut h = Hand::new();
        h.add( Card{ rank: CardRank::Seven, suite: CardSuite::Clubs } );
        h.add( Card{ rank: CardRank::Seven, suite: CardSuite::Spades } );
        let mut d = Hand::new();
        d.add( Card{ rank: CardRank::Ace, suite: CardSuite::Clubs } );
        d.add( Card{ rank: CardRank::Two, suite: CardSuite::Spades } );
        d.add( Card{ rank: CardRank::Three, suite: CardSuite::Spades } );
        d.add( Card{ rank: CardRank::Four, suite: CardSuite::Spades } );
        d.add( Card{ rank: CardRank::Five, suite: CardSuite::Spades } );

        let c = form_all_candidates(h, d);
        assert_eq!( c.len(), 21 );
    }
}
