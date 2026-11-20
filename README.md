# CS19300 project
Contains my personal projects required for Purdue CS 19300 | a command line program to help users learn how to count cards.

### Project Goal
Create a Blackjack simulater/training program that helps people learn how to count cards and practice their skills via the Hi-Lo system
## Tech Stack
* Github pages for hosting (or host on neociites)
	* WebAssembly (wasm-pack or trunk) to compile code into javascript
* Rust
* Crossterm or Ratatui (for nicer command line interactions)
	- HTML and CSS if I have time to make a UI
* JSON for data storage
* Git for version control

## MVP Scope
* Full gameplay simulation utilizing a standard 52-card deck.
* Ability to configure and play with a varying number of decks.
* Live updates of the card count using the Hi-Lo counting system.
* Some tracker displaying how much of the deck is left (toggelable)

## Future Plans
* Exercises to help players memorize the count and learn optimal play lines.
* Tools to simulate betting strategies and track long-term bankroll management.
* Detailed statistics and progress tracking metrics for user improvement over time.
* Common blackjack side bets including Insurance, Perfect Pairs, and 21+3.
* Display statistical player edge based on the current count.
* Competitive ranking system to compare tracking efficiency and scores with other players worldwide.

### Docs
A card is a struct of 2 enums (suit and rank) and an impl with methods to get the suit and rank, a to string and constructor
A deck is a struct of 2 lists containing played cards and cards that need to be delt (playable cards) and 
an impl with a deal that randomly picks a card and moves it to the played list (Vec) and and a constructor that 
initializes the 2 decks and fills the playable deck with all the cards
