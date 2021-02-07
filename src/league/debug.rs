use super::game::Race;
use super::league::League;


impl std::fmt::Display for League {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let gpw = League::matches_per_week_static(self.players.len());

        let mut week: usize = 0;

        let empty_line = |f: &mut std::fmt::Formatter| -> std::fmt::Result {
            for _ in 0..gpw {
                write!(f, "|                     ")?;
            }
            write!(f, "|\n")
        };

        let print_player_line = |f: &mut std::fmt::Formatter, week: &usize, first: bool| -> std::fmt::Result {
            for g in 0..gpw {
                let name = match first {
                    true => &self.players[self.matches[week * gpw + g].players.0].name,
                    false => &self.players[self.matches[week * gpw + g].players.1].name,
                };


                let m = &self.matches[week * gpw + g];
                let (left_decorator, right_decorator) = match m.winner() {
                    Some(winner) => match winner == m.players.0 && first {
                        true => (">", "<"),
                        false => (" ", " ")
                    },
                    None => (" ", " ")
                };

                write!(f, "| {decor}{name:namewidth$}   ", decor = left_decorator, name = name, namewidth = 11)?;
                for i in 0..3 {
                    let game = m.games.get(i);
                    let mut c = match game {
                        Some(t) => match match first {
                            true => &t.races.0,
                            false => &t.races.1
                        } {
                            Race::Terran => 't',
                            Race::Zerg => 'z',
                            Race::Protoss => 'p',
                        },
                        None => ' '
                    };

                    match game {
                        Some(t) => if t.first_player_won == first { c = c.to_ascii_uppercase() },
                        None => ()
                    }

                    write!(f, "{}", c)?;
                }
                write!(f, "{} ", right_decorator)?;
            }
            write!(f, "|\n")
        };

        loop {
            for _ in 0..gpw {
                write!(f, "+---------------------")?;
            }
            write!(f, "+\n")?;

            if week == League::weeks_count_static(self.players.len()) {
                break;
            }

            empty_line(f)?;

            print_player_line(f, &week, true)?;
            print_player_line(f, &week, false)?;

            empty_line(f)?;


            week += 1
        }

        write!(f, "")
    }
}
