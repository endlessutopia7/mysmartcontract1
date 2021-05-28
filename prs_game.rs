extern crate rand;
use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};

pub mod paperrockscissor {
    pub struct MatchRecords {
        player1wins: u8,
        player2wins: u8,
    }

    impl MatchRecords {
        pub fn new(initwins1: u8, initwins2: u8) -> Self {
            Self { player1wins: initwins1, player2wins: initwins2 }
        }

        pub fn default() -> Self {
            Self::new(Default::default())
        }

        pub fn getplayerchoice() -> u8 {
            let mut rng = rand::thread_rng();
            let die = Uniform::from(0..3); // 0 for paper, 1 for rock, 2 for scissors   
            let throw = die.sample(&mut rng);
            
            throw
        }

        pub fn decidewhowinsgame(player1choice: u8, player2choice: u8) -> str {
            if player1choice == player2choice
            {
                return "Draw";
            }
            else if (player1choice != 2 && player1choice < player2choice) || (player1choice == 2 && player2choice == 0)
            {
                return "Player 1";
            }
            else
            {
                return "Player 2";
            }
        }

        pub fn decidewhowinsmatch(&self) -> str {
            if self.player1wins == self.player2wins { "Draw" } else { if self.player1wins > self.player2wins { "Player 1" } else {"Player 2"} }
        }

        pub fn playawholematchofgames(&mut self, gameset: u8)
        {
            for n in 1..gameset
            {
                let currentplayer1choice = self.getplayerchoice();
                let currentplayer2choice = self.getplayerchoice();
                if (self.decidewhowinsgame() == "Player 1")
                {
                    self.player1wins += 1;
                }
                else if (self.decidewhowinsgame() == "Player 2")
                {
                    self.player2wins += 1;
                }
            }
        }

        pub fn getplayer1wins(&self) -> u8 {
            self.player1wins
        }

        pub fn getplayer2wins(&self) -> u8 {
            self.player2wins
        }
    }

    mod tests {
        use super::*;

        #[test]
        fn default_initialization() {
            let matchrecords = MatchRecords::default();
            assert_eq!(matchrecords.getplayer1wins(), 0);
            assert_eq!(matchrecords.getplayer2wins(), 0);
        }

        #[test]
        fn valid_choice() {
            let mut matchrecords = MatchRecords::default();
            let choice = matchrecords.getplayerchoice();
            assert_eq!(choice == 0 || choice == 1 || choice == 2, true); // must be 0, 1, or 2 (P, R, S)

            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }

        #[test]
        fn single_game_result_validation() {
            let mut matchrecords = MatchRecords::default();

            // choices are hard-coded for convenience as actual result is random and occasional

            // when player 1 wins
            assert_eq!(matchrecords.decidewhowinsgame(0, 1), "Player 1");
            assert_eq!(matchrecords.decidewhowinsgame(1, 2), "Player 1");
            assert_eq!(matchrecords.decidewhowinsgame(2, 0), "Player 1");

            // when draw
            assert_eq!(matchrecords.decidewhowinsgame(0, 0), "Draw");
            assert_eq!(matchrecords.decidewhowinsgame(1, 1), "Draw");
            assert_eq!(matchrecords.decidewhowinsgame(2, 2), "Draw");

            // when player 2 wins
            assert_eq!(matchrecords.decidewhowinsgame(1, 0), "Player 1");
            assert_eq!(matchrecords.decidewhowinsgame(2, 1), "Player 1");
            assert_eq!(matchrecords.decidewhowinsgame(0, 2), "Player 1");
        }

        #[test]
        fn whole_match_result_validation() {
            let mut matchrecords = MatchRecords::default();
            matchrecords.playawholematchofgames(5);
            let resultdiff = matchrecords.getplayer1wins() - matchrecords.getplayer2wins();
            let expectedwinner = if resultdiff == 0 { "Draw" } else { if resultdiff > 0 {"Player 1"} else {"Player 2"}};

            assert_eq!(matchrecords.decidewhowinsmatch(), expectedwinner);
        }
    }
}