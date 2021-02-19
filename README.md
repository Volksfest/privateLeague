# privateLeague

Learning Rust by making a private league management software.

It has shitty naming, shitty design but well, it gets to something... 

# features

- server/client web architecture
- check for consistency with multiple clients at the same time
- generate a league
- add games to a match
- remove all games from a match
- calculate leaderboard

# generation

The league generates a league "config" file (first positional argument) if the specified file does not exist.
For the generation, a valid list of player names has to be given (everything after --players).

The binded host address (default is localhost:8080) can be specified with --host