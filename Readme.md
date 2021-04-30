This is a simple implementation of a connect-4 game in Rust, with a basic artificial intelligence (AI). The board has 7 columns and 6 lines.

My main goal when writing this game was to get some familiarity with basic features of the Rust language. As such, I did not test it extensively (yet), and it is likely that some bugs and inefficiencies remain.

## Use

This game currently works only in a terminal or terminal emulator with support for box-drawing and disc characters. If you wish to play it in a terminal which does not support these characters, you can either replace them by ASCII characters in the `board.rs` file (function `show_board`) or replace the function `show_board` by the (currently commented out) function `show_board_ascii` one before compiling. I do not plan to build a graphical interface at the moment (but any idea to improve how the board looks is warmly welcome!)

In a terminal, type `./connect_4` to launch the game. You are then asked whether to play with an AI. Type `1` to have an AI as first player, `2` to make it the second player, `12` to have two AIs play against each other, or anything else to have no AI (you can then play against a human friend on the same computer, or against yourself if you are so inclined). 

If one AI is present, you may choose its strength as a positive integer. This only affects how many turns it looks ahead to decide whether it or its opponent has a winning strategy. If it can't find such a strategy, it reverts to an estimate of the value of each possible move independent of its strength. A value higher than 5 may make the game quite slow.

At the beginning of each turn, the current board will be printed. If it is a player's turn, they can play by typing a number from 1 to 8, corresponding to the column they would like to play. 

The game ends when one player has four (or more) aligned pieces or when the board is full.

## Build

To build this game, you need a Rust compiler (probably at least version 1.41.0; I tested it with rustc version 1.51.0). If you have cargo installed, you may build it by running `cargo build --release`. The executable will be placed in the folder `target/release`. 

## FAQ

*Is the IA good?*

I guess it depends on your reference frame. It is good enough to beat me most of the times I play against it, but it probably tells more about my lack of experience with the connect-4 game than anything else. More seriously, it only checks if a winning strategy exists for the next few rounds and reverts to simple (and not yet optimized) algorithm to estimate the value of each possible move if there is none, so I do not expect it to be very strong in absolute terms. 

*Which value should I choose for the IA strength?*

It's really up to you. I would advise against values higher than 5 or 6 unless you have a really fast computer or are happy to wait  a long time between turns. 

*Can I ask a question/submit ideas for improvement/offer criticism?*

Absolutely! I made this game to learn (about Rust, and maybe a bit about connect-4), and I'm always glad to receive comments, constructive criticism, or exchange points of views! 

*Can I use your code for my project?*

Sure! Please just bear in mind that it is not really optimized—in particular, the AI could probably be made much faster and efficient. So it may be worth looking for other implementations first (especially if they are written by people who know how to play connect-4 competitively, which I definitely don't), which may provide a better starting point.

*Can you implement feature x?*

If you would like to seem something added to this project, please feel free to ask! I can't guarantee I will have the time of know-how to do it, but I'm always happy to consider any suggestion!
