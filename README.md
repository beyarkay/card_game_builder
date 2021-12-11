# Card Game Builder

### Convert card game ideas into printable PDFs you can cut out!

Lots of card games boil down to basically just a load of creative prompts on
cardboard which you. Think 
[Cards Against Humanities](https://www.cardsagainsthumanity.com/), 
[Chameleon](https://bigpotato.com/products/the-chameleon), 
[Red Flags](https://www.timelessboardgames.co.za/boardgames/red-flags-main-game/1985), etc.

This project aims to make it simple to create your own card games by writing up
your prompts in a file. The Rust program will then convert that file to a PDF
with nicely formatted cards which you can print off, cut out, and then play
with.

## Features

- Open source
- Multiple card games are already in the repository
- Easy (and flexible) interface via YAML markup files

## Installation

Assuming you've [installed git](https://git-scm.com/downloads) and have also
[installed rust](https://www.rust-lang.org/tools/install), simply `git clone`
and then `cargo run`:

1. `git clone https://github.com/beyarkay/card_game_builder.git`
2. `cd card_game_builder`
3. `cargo run`

## PDF file to playable card game

The best way I've found is to take the PDF to a print shop, get it printed out
on card, and then ask to use their guillotine (or ask if they'll cut it up for
you).

Once you've got all your cards cut out, I _highly_ recommend buying a box or
two of small circle stickers and putting one sticker around the corner of each
playing card. This will make it obvious which game it is (ie a red circle for
the game _Red Flags_ or a black dot for _Cards Against Humanity_) and also make
it much easier to keep all the cards the right way up.

You can also often get long thin plastic boxes from a plastics warehouse which
are perfect for holding the cards.

## Creating your own card game

The Rust code expects to be given a `.yaml` file, and will write one or more
`.pdf` files. This `.yaml` file is what you'll write to describe your game, and
must include _everything_ about that game, including:

- The name of the game and version number
- A list of _categories_, where each category can be treated differently and
  cards within a category are printed sequentially
- A list of cards (one list per category), where each card defines the text
  that will be on that card.

For example `cards_against_humanities.yaml`: 
```yaml
name: Cards Against Humanities
version: 1.0
categories:
  - white_cards:
    - value1
    - value2
    - value3
  - black_cards:
    - value1
    - value2
    - value3
```

The code will enforce that the game name, at least one category, and at least
one card exist. Beyond that, there are optional extras that make the game more
playable and fun:

- A short description of the game
- Short instructions on how to play (should fit on one card)
- Number of players
- Duration of the game
- Author(s)
- Website link

The above information will be shown on the first page of the PDF, on the first
couple of cards. The date of creation will also be printed on the first few
cards.

If you've got a lot (over 500) cards, then printing them out all at once might
be too expensive. You can define _editions_ of your card game, such that each
edition is mutually exclusive. This allows you to first create a small batch of
cards, print them out, and then if you want to create more cards you can define
them as a second edition in the same `.yaml` file and tell the program to only
print out the second edition.

Or alternatively you could treat the different editions as themes for the game,
like _The Halloween Edition_, _The Sexy Edition_, _The University Edition_, 
_The "People I've Dated" Edition_, etc.

## YAML Card Game File to PDF

This is as simple as running the Rust code and passing in the path to your
`.yaml` file. 
```sh
cargo run card_game_builder games/cards_against_humanities.yaml
```

You can optionally pass in the edition(s) you'd like to generate as quoted
strings:
```sh
cargo run card_game_builder games/cards_against_humanities.yaml "The Sexy
Edition" "The Chrismas Edition"
```

## Contributions

Contributions are very welcome! And they don't have to be code either. If
you've got a card game file that you'd like to contribute, then submit a pull
request and I'll add it.

Features, bug fixes, and improvements are also welcome!

