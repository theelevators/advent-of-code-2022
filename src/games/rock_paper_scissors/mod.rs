use std::fs::read_to_string;


pub type GameScore = i32;
pub type Predict = String;
pub type Predictions = Vec<Prediction>;
pub type Games = Vec<RockPaperScissors>;

#[derive(Clone, Debug)]
pub struct Guide {
    guide: Games,
    predictions: Option<Predictions>,
}

impl Guide {
    pub fn new(file: &str) -> Guide {
        Guide { guide: Games::new_tournament(file), predictions: None }
    }

    pub fn new_predictions(mut self, file: &str) -> Self {
        self.predictions = Some(Games::prediction_outcomes(file));
        self
    }
    pub fn follow_my_guide(self) -> GameScore {
        let mut score: GameScore = 0;

        for game in self.guide {
            let my_hand = game.c2;
            let opp = game.c1;
            match my_hand {
                HandChoice::Rock(val) => {
                    match my_hand.duel(&opp) {
                        GameResults::Win(s) => {
                            score += s + val;
                        }
                        GameResults::Draw(s) => {
                            score += s + val;
                        }
                        GameResults::Loss(s) => {
                            score += s + val;
                        }
                    }
                }
                HandChoice::Paper(val) => {
                    match my_hand.duel(&opp) {
                        GameResults::Win(s) => {
                            score += s + val;
                        }
                        GameResults::Draw(s) => {
                            score += s + val;
                        }
                        GameResults::Loss(s) => {
                            score += s + val;
                        }
                    }
                }
                HandChoice::Scissors(val) => {
                    match my_hand.duel(&opp) {
                        GameResults::Win(s) => {
                            score += s + val;
                        }
                        GameResults::Draw(s) => {
                            score += s + val;
                        }
                        GameResults::Loss(s) => {
                            score += s + val;
                        }
                    }
                }
            }
        }
        score
    }
    pub fn follow_elfs_guide(self) -> GameScore {
        let mut score: GameScore = 0;

        for prediction in self.predictions.unwrap() {
            let hand: HandChoice = prediction.hand;
            let predict: Predict = prediction.predict;

            match GameResults::prediction(predict.as_str()) {
                GameResults::Win(s) => {
                    match hand.loses_against() {
                        HandChoice::Rock(val) => {
                            score += val + s;
                        }
                        HandChoice::Paper(val) => {
                            score += val + s;
                        }
                        HandChoice::Scissors(val) => {
                            score += val + s;
                        }
                    }
                }
                GameResults::Loss(s) => {
                    match hand.wins_against() {
                        HandChoice::Rock(val) => {
                            score += val + s;
                        }

                        HandChoice::Paper(val) => {
                            score += val + s;
                        }

                        HandChoice::Scissors(val) => {
                            score += val + s;
                        }
                    }
                }
                GameResults::Draw(s) => {
                    match hand.draws_againts() {
                        HandChoice::Rock(val) => {
                            score += val + s;
                        }
                        HandChoice::Paper(val) => {
                            score += val + s;
                        }
                        HandChoice::Scissors(val) => {
                            score += val + s;
                        }
                    }
                }
            }
        }
        score
    }
}

pub trait Tournament {
    fn new_tournament(file: &str) -> Games;
    fn prediction_outcomes(file: &str) -> Predictions;
}

impl Tournament for Games {
    fn new_tournament(file: &str) -> Games {
        let mut tournament = Games::new();
        for line in read_to_string(file).unwrap().lines() {
            let c1 = HandChoice::new(line.chars().nth(0).unwrap().to_string().as_str());
            let c2 = HandChoice::new(line.chars().nth(2).unwrap().to_string().as_str());
            tournament.push(RockPaperScissors::new(c1, c2));
        }
        tournament
    }
    fn prediction_outcomes(file: &str) -> Predictions {
        let mut predictions:Predictions = Predictions::new();
        for line in read_to_string(file).unwrap().lines() {
            let hand: HandChoice = HandChoice::new(
                line.chars().nth(0).unwrap().to_string().as_str()
            );
            let predict: Predict = Predict::from(line.chars().nth(2).unwrap());
            predictions.push(Prediction::new(hand, predict));
        }
        predictions
    }
}

#[derive(Clone, Debug)]
pub struct Prediction {
    pub hand: HandChoice,
    pub predict: Predict,
}

impl Prediction {
    pub fn new(hand: HandChoice, predict: Predict) -> Prediction {
        Prediction { hand, predict }
    }
}

#[derive(Clone, Debug)]
pub struct RockPaperScissors {
    pub c1: HandChoice,
    pub c2: HandChoice,
}

impl RockPaperScissors {
    pub fn new(c1: HandChoice, c2: HandChoice) -> RockPaperScissors {
        RockPaperScissors { c1, c2 }
    }
}

#[derive(Clone, Debug)]
pub enum HandChoice {
    Rock(GameScore),
    Paper(GameScore),
    Scissors(GameScore),
}
impl HandChoice {
    pub fn new(choice: &str) -> HandChoice {
        match choice {
            "A" => HandChoice::Rock(1),
            "B" => HandChoice::Paper(2),
            "C" => HandChoice::Scissors(3),
            "X" => HandChoice::Rock(1),
            "Y" => HandChoice::Paper(2),
            "Z" => HandChoice::Scissors(3),
            &_ => panic!("Invalid Choice!"),
        }
    }
    pub fn duel(&self, opp: &HandChoice) -> GameResults {
        match self {
            HandChoice::Paper(..) => {
                match opp {
                    HandChoice::Paper(..) => GameResults::Draw(3),
                    HandChoice::Rock(..) => GameResults::Win(6),
                    HandChoice::Scissors(..) => GameResults::Loss(0),
                }
            }
            HandChoice::Rock(..) => {
                match opp {
                    HandChoice::Paper(..) => GameResults::Loss(0),
                    HandChoice::Rock(..) => GameResults::Draw(3),
                    HandChoice::Scissors(..) => GameResults::Win(6),
                }
            }
            HandChoice::Scissors(..) => {
                match opp {
                    HandChoice::Paper(..) => GameResults::Win(6),
                    HandChoice::Rock(..) => GameResults::Loss(0),
                    HandChoice::Scissors(..) => GameResults::Draw(3),
                }
            }
        }
    }

    pub fn wins_against(&self) -> HandChoice {
        match self {
            HandChoice::Paper(..) => HandChoice::Rock(1),
            HandChoice::Rock(..) => HandChoice::Scissors(3),
            HandChoice::Scissors(..) => HandChoice::Paper(2),
        }
    }
    pub fn loses_against(&self) -> HandChoice {
        match self {
            HandChoice::Scissors(..) => HandChoice::Rock(1),
            HandChoice::Paper(..) => HandChoice::Scissors(3),
            HandChoice::Rock(..) => HandChoice::Paper(2),
        }
    }
    pub fn draws_againts(&self) -> HandChoice {
        match self {
            HandChoice::Rock(..) => HandChoice::Rock(1),
            HandChoice::Paper(..) => HandChoice::Paper(2),
            HandChoice::Scissors(..) => HandChoice::Scissors(3),
        }
    }
}

pub enum GameResults {
    Win(GameScore),
    Loss(GameScore),
    Draw(GameScore),
}

impl GameResults {
    pub fn prediction(opp: &str) -> GameResults {
        match opp {
            "X" => GameResults::Loss(0),
            "Y" => GameResults::Draw(3),
            "Z" => GameResults::Win(6),
            &_ => panic!("Invalid Choice!"),
        }
    }
}
