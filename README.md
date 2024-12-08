# Lottery_Randomization
Rust applications to apply randomization strategies on drawing lottery numbers

Application connects to a Postgresql instance running in a container. This database contains the table with all of the drawn numbers for both Powerball and the Mega Millions lottery games since the inception of the latest ball numbering format. The base application uses a simple random selection from the drawn numbers as its weighting of both the 5 regular balls and the 1 money ball.

The application uses a fixed size number to create a list of draws to simulate the likelihood of the numbers being drawn in a winning sequence. These values are then stored in the Draws tables for the respective games.

Project uses the postgresql cargo package r2d2 for connectivity. The rust compiler is running in a container with the latest version installed in a Linux environment.

