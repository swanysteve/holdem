# holdem
Deals Texas Hold'em poker hands and decides which is the winner.

Example:
```
{
% cargo run --quiet
Player 1: 2♠ 9♠
Player 2: 10♦ 10♣
Dealer: 5♥ 5♠ 6♣ 9♥ Q♣
  Best hand for Player 1:  Two Pair, High Rank: 9♠, Low Rank: 5♥, Kicker: Q♣
  Best hand for Player 2:  Two Pair, High Rank: 10♦, Low Rank: 5♥, Kicker: Q♣
Player 2 wins.
}
```
