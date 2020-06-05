A project aiming to build Shadowverse decks through a genetic algorithm. The goal and challenge is to implement an evaluation function whose fitness value is the winrate against human made decks, making use of a (sensibly) playing AI.

## Requirements
- [python3](https://www.python.org/downloads/)
- matplotlib (```pip install matplotlib``` in a terminal/cmd)
- [Bagoum's json](https://sv.bagoum.com/cardsFullJSON/en), if not up to date.

## Usage
- Download the repository and unzip it.
- Navigate to the unzipped directory and run ```python main.py``` in a terminal/cmd.
- You can end the program with Ctrl+C once you're satisfied with the deck score shown.

## Project structure
Indented by imports:

- main.py
  - Gene.py
    - Deck.py
      - cardlib.py
        - tagger.py

## TODO
- A simulation framework for the AI to play in (in a compiled language).
- Hardcoding cards for being handled by the above framework.
- A playing AI to play with the above cards.
- A winrate based evaluation function using the above AI.
