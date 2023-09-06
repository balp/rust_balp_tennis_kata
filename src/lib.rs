pub trait TennisGame {
    fn won_point(&mut self, player_name: &str);
    fn get_score(&self) -> String;
}

#[derive(Default)]
struct TennisGame1 {
    player_1_score: u8,
    player_2_score: u8,
}

fn score_name(score: u8) -> String {
    match score {
        3 => "Forty".to_string(),
        2 => "Thirty".to_string(),
        1 => "Fifteen".to_string(),
        0 => "Love".to_string(),
        _ => "".to_string(),
    }
}

impl TennisGame for TennisGame1 {
    fn won_point(&mut self, player_name: &str) {
        if player_name == "player1" {
            self.player_1_score += 1;
        } else {
            self.player_2_score += 1;
        }
    }

    fn get_score(&self) -> String {
        match (self.player_1_score, self.player_2_score) {
            // A game is won by the first player to have won at least four points in total and
            // at least two points more than the opponent.
            (x, y) if x >= (y + 2) && x >= 4 => "Win for player1".to_string(),
            (x, y) if y >= (x + 2) && y >= 4 => "Win for player2".to_string(),

            // If at least three points have been scored by each side and a player has one more
            // point than his opponent, the score of the game is “advantage” for the player
            // in the lead.
            (x, y) if x > y && x >= 4 => "Advantage player1".to_string(),
            (x, y) if x < y && y >= 4 => "Advantage player2".to_string(),

            // If at least three points have been scored by each player, and the scores are
            // equal, the score is “deuce”.
            (x, y) if x == y && x >= 3 => "Deuce".to_string(),

            // The running score of each game is described in a manner peculiar to tennis:
            // scores from zero to three points are described as “love”, “15”, “30”, and
            // “40” respectively.
            (x, y) if x == y => score_name(x) + "-All",
            (x, y) => score_name(x) + "-" + &score_name(y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! tennis_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (p1score, p2score, expected) = $value;
                let mut game = TennisGame1::default();
                let highest_score = u8::max(p1score, p2score);
                for i in 0..highest_score {
                    if i < p1score {
                        game.won_point("player1");
                    }
                    if i < p2score {
                        game.won_point("player2");
                    }
                }
                assert_eq!(game.get_score(), expected, "{},{}", p1score, p2score);
            }
        )*
        }
    }

    tennis_tests! {
        tennis_0_0: (0, 0, "Love-All"),
        tennis_1_1: (1, 1, "Fifteen-All"),
        tennis_2_2: (2, 2, "Thirty-All"),
        tennis_3_3: (3, 3, "Deuce"),
        tennis_4_4: (4, 4, "Deuce"),
        tennis_1_0: (1, 0, "Fifteen-Love"),
        tennis_1_2: (1, 2, "Fifteen-Thirty"),
        tennis_1_3: (1, 3, "Fifteen-Forty"),
        tennis_2_0: (2, 0, "Thirty-Love"),
        tennis_2_1: (2, 1, "Thirty-Fifteen"),
        tennis_2_3: (2, 3, "Thirty-Forty"),
        tennis_3_0: (3, 0, "Forty-Love"),
        tennis_3_1: (3, 1, "Forty-Fifteen"),
        tennis_3_2: (3, 2, "Forty-Thirty"),
        tennis_0_1: (0, 1, "Love-Fifteen"),
        tennis_0_2: (0, 2, "Love-Thirty"),
        tennis_0_3: (0, 3, "Love-Forty"),
        tennis_4_3: (4, 3, "Advantage player1"),
        tennis_3_4: (3, 4, "Advantage player2"),
        tennis_5_4: (5, 4, "Advantage player1"),
        tennis_4_5: (4, 5, "Advantage player2"),
        tennis_15_14: (15, 14, "Advantage player1"),
        tennis_14_15: (14, 15, "Advantage player2"),
        tennis_4_0: (4, 0, "Win for player1"),
        tennis_0_4: (0, 4, "Win for player2"),
        tennis_6_4: (6, 4, "Win for player1"),
        tennis_4_6: (4, 6, "Win for player2"),
        tennis_16_14: (16, 14, "Win for player1"),
        tennis_14_16: (14, 16, "Win for player2"),
    }
}
