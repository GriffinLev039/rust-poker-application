#[derive(Debug, Clone)]
pub struct Pot {
    chip_count: u32,
    side_pots: Vec<u32>,
}

impl Default for Pot {
    fn default() -> Self {
        Pot {
            chip_count: 0,
            side_pots: vec![],
        }
    }
}

impl Pot {
    pub fn add_chips(&mut self, arg_chips: u32) {
        self.chip_count += arg_chips;
    }
    pub fn empty_pot(&mut self) {
        self.side_pots = vec![];
        self.chip_count = 0;
    }
    pub fn get_chips(&self) -> u32 {
        return self.chip_count;
    }
    pub fn create_side_pot(&mut self, value: u32) {
        self.side_pots.push(value);
    }
    // Fix cloning later
    pub fn get_side_pots(&self) -> Vec<u32> {
        self.side_pots.clone()
    }
    /*
     * The idea here is that it'll return a vector containing the difference between the main pot and side pots iteratively.
     * Example:
     * THIS NEEDS TO BE SORTED!!!!!
     * What the fuck was the point of this actually
     */
    //     pub fn get_pot_differences(&self) -> Vec<u32> {
    //         let all_pots: Vec<u32> = std::iter::once(self.chip_count)
    //             .chain(self.get_side_pots().into_iter())
    //             .collect();
    //         let mut diff_vec: Vec<u32> = vec![0; all_pots.len()];
    //         for i in 0..all_pots.len() - 1 {
    //             diff_vec[i] = all_pots[i] - all_pots[i + 1];
    //         }
    //         diff_vec[all_pots.len() - 1] = all_pots[0] - all_pots[all_pots.len() - 1];
    //         diff_vec
    //     }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn add_chips_test() {
        let mut pot_struct = Pot::default();
        let past_pot_val = pot_struct.get_chips();
        pot_struct.add_chips(1000);
        assert_eq!(pot_struct.get_chips() - past_pot_val, 1000);
    }
    // #[test]
    // fn side_pots_test_half() {
    //     let mut pot_struct = Pot::default();
    //     pot_struct.chip_count = 3000;
    //     pot_struct.create_side_pot(1500);
    //     println!("{:?}", pot_struct);
    //     println!("{:?}", pot_struct.get_pot_differences());
    //     assert_eq!(pot_struct.get_pot_differences(), vec![1500, 1500]);
    // }
    // #[test]
    // fn side_pots_test_staggered() {
    //     let mut pot_struct = Pot::default();
    //     pot_struct.add_chips(3000);
    //     pot_struct.create_side_pot(1000);
    //     pot_struct.create_side_pot(500);
    //     assert_eq!(pot_struct.get_pot_differences(), vec![1500, 1000, 500]);
    // }
}
