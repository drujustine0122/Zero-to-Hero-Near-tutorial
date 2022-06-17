use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

const PUZZLE_NUMBER:u8 = 1;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    crossowrd_solution: String,
}

#[near_bindgen]
impl Contract {
    pub fn get_puzzle_number(&self) -> u8 {
        PUZZLE_NUMBER
    }

    pub fn set_solution(&mut self, solution: String) {
        self.crossowrd_solution = solution;
    }

    pub fn guess_solution(&mut self, solution: String) {
        if solution == self.crossowrd_solution {
            env::log_str("You guessed right!")
        } else {
            env::log_str("Try again")
        }
    }
}

#[cfg(test)]
mod tests {
    
    use near_sdk::test_utils::{VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }
}