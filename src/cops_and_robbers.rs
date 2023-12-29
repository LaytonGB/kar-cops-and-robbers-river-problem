use crate::boat_position::BoatPosition;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopsAndRobbers {
    pub cops: i8,
    pub robbers: i8,
    pub boat: BoatPosition,
}

impl CopsAndRobbers {
    pub fn new() -> Self {
        CopsAndRobbers {
            cops: 3,
            robbers: 3,
            boat: BoatPosition::Left,
        }
    }

    fn validate(&self, other: &Self) -> bool {
        if self.boat == other.boat {
            return false;
        }

        let (cops_l, cops_r) = (other.cops, 3 - other.cops);
        let (robbers_l, robbers_r) = (other.robbers, 3 - other.robbers);
        let total_change = (self.cops - other.cops).abs() + (self.robbers - other.robbers).abs();

        if total_change > 2 || total_change == 0 {
            return false;
        }
        if cops_l < 0 || cops_l > 3 || cops_r < 0 || cops_r > 3 {
            return false;
        }
        if robbers_l < 0 || robbers_l > 3 || robbers_r < 0 || robbers_r > 3 {
            return false;
        }
        if cops_l < robbers_l && cops_l != 0 {
            return false;
        }
        if cops_r < robbers_r && cops_r != 0 {
            return false;
        }
        true
    }

    pub fn get_valid_moves(self) -> Vec<CopsAndRobbers> {
        let mut moves = Vec::new();
        let cops = self.cops;
        let robbers = self.robbers;

        for i in !1..=2 {
            for j in !1..=2 {
                let new_cops = cops + i;
                let new_robbers = robbers + j;
                let new_state = CopsAndRobbers {
                    cops: new_cops,
                    robbers: new_robbers,
                    boat: self.boat.switch(),
                };
                if self.validate(&new_state) {
                    moves.push(new_state);
                }
            }
        }

        moves
    }
}

impl Ord for CopsAndRobbers {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (3 - self.robbers).cmp(&(3 - other.robbers))
    }
}

impl PartialOrd for CopsAndRobbers {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
