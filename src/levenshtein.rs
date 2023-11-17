use std::cmp::min;
use std::f32;

#[derive(Debug)]
pub struct Differencer {
    insert: f32,
    delete: f32,
    substitution: f32,
}

impl Differencer {
    pub fn standard() -> Self {
        Self {
            insert: 1.0,
            delete: 1.0,
            substitution: 1.0,
        }
    }

    pub fn new(insert: f32, delete: f32, substitution: f32) -> Self {
        Self {
            insert,
            delete,
            substitution,
        }
    }

    pub fn score(&self, source: &str, target: &str) -> f32 {
        let width = source.len();

        let mut row0 = vec![0.0; width + 1];
        let mut row1 = vec![0.0; width + 1];

        // base case
        for (ii, etl) in row0.iter_mut().enumerate() {
            *etl = ii as f32 * self.insert;
        }

        let mut last_row = 0;
        for (ii, t_char) in target.chars().enumerate() {
            let row = ii + 1;
            if row % 2 == 1 {
                row1[0] = (row as f32) * self.delete;
                self.fill_row(t_char, &source, &row0, &mut row1);
                last_row = 1;
                //println!("{:?}", row1);
            } else {
                row0[0] = row as f32 * self.delete;
                self.fill_row(t_char, &source, &row1, &mut row0);
                last_row = 0;
                //println!("{:?}", row0);
            }
        }

        if last_row == 0 {
            row0[row0.len() - 1]
        } else {
            row1[row1.len() - 1]
        }
    }

    fn fill_row(&self, t_char: char, src: &str, prev_row: &[f32], cur_row: &mut [f32]) {
        for (jj, s_char) in src.chars().enumerate() {
            let col = jj + 1;
            let sub_cost: f32 = if t_char == s_char {
                0.0
            } else {
                self.substitution
            };

            let delete_cost = cur_row[col - 1] + self.delete;
            let insert_cost = prev_row[col] + self.insert;
            let sub_cost = prev_row[col - 1] + sub_cost;
            let cell_cost = f32::min(f32::min(delete_cost, insert_cost), sub_cost);
            cur_row[col] = cell_cost;
        }
    }
}

fn fill_row(t_char: char, src: &str, prev_row: &[usize], cur_row: &mut [usize]) {
    for (jj, s_char) in src.chars().enumerate() {
        let col = jj + 1;
        let sub_cost = if t_char == s_char { 0 } else { 1 };

        let delete_cost = cur_row[col - 1] + 1;
        let insert_cost = prev_row[col] + 1;
        let sub_cost = prev_row[col - 1] + sub_cost;
        let cell_cost = min(min(delete_cost, insert_cost), sub_cost);
        cur_row[col] = cell_cost;
    }
}

pub fn twovec_lev(src: &str, target: &str) -> usize {
    let width = src.len();

    let mut row0 = vec![0; width + 1];
    let mut row1 = vec![0; width + 1];

    // base cases
    for (ii, etl) in row0.iter_mut().enumerate() {
        *etl = ii;
    }

    let mut last_row = 0;
    //println!("{:?}", row0);
    // fill in table
    for (ii, t_char) in target.chars().enumerate() {
        let row = ii + 1;
        //println!("row:{}", row);
        if row % 2 == 1 {
            row1[0] = row;
            fill_row(t_char, &src, &row0, &mut row1);
            last_row = 1;
            //println!("{:?}", row1);
        } else {
            row0[0] = row;
            fill_row(t_char, &src, &row1, &mut row0);
            last_row = 0;
            //println!("{:?}", row0);
        }
    }

    if last_row == 0 {
        row0[row0.len() - 1]
    } else {
        row1[row1.len() - 1]
    }
}

pub fn vecvec_lev(src: &str, target: &str) -> usize {
    let width = src.len();
    let height = target.len();

    let mut grid = vec![vec![0; width + 1]; height + 1];

    // base cases
    for (ii, etl) in grid[0].iter_mut().enumerate() {
        *etl = ii;
    }
    for (ii, row) in grid.iter_mut().enumerate() {
        row[0] = ii;
    }

    // fill in table
    for (ii, t_char) in target.chars().enumerate() {
        let row = ii + 1;
        //println!("row:{}", row);
        for (jj, s_char) in src.chars().enumerate() {
            let col = jj + 1;
            let sub_cost = if t_char == s_char { 0 } else { 1 };

            let delete_cost = grid[row][col - 1] + 1;
            let insert_cost = grid[row - 1][col] + 1;
            let sub_cost = grid[row - 1][col - 1] + sub_cost;
            let cell_cost = min(min(delete_cost, insert_cost), sub_cost);
            grid[row][col] = cell_cost;
        }
    }

    //for row in grid.iter() {
    //println!("{:?}", row);
    //}

    // answer is final cell of grid
    grid[grid.len() - 1][grid[0].len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vecvec_lev_exact() {
        let source: String = "abc".into();
        let target: String = "abc".into();

        let result = vecvec_lev(&source, &target);
        assert_eq!(result, 0);
    }

    #[test]
    fn vecvec_lev_abc_v_bca() {
        let source: String = "abc".into();
        let target: String = "bca".into();

        let result = vecvec_lev(&source, &target);
        assert_eq!(result, 2);
    }

    #[test]
    fn vecvec_lev_one_empty() {
        let source: String = "abc".into();
        let target: String = "".into();

        let result = vecvec_lev(&source, &target);
        assert_eq!(result, source.len());

        let source: String = "".into();
        let target: String = "abc".into();

        let result = vecvec_lev(&source, &target);
        assert_eq!(result, target.len());
    }

    #[test]
    fn twovec_lev_exact() {
        let source: String = "abc".into();
        let target: String = "abc".into();

        let result = twovec_lev(&source, &target);
        assert_eq!(result, 0);
    }

    #[test]
    fn twovec_lev_abc_v_bca() {
        let source: String = "abc".into();
        let target: String = "bca".into();

        let result = twovec_lev(&source, &target);
        assert_eq!(result, 2);
    }

    #[test]
    fn twovec_lev_one_empty() {
        let source: String = "".into();
        let target: String = "abc".into();

        let result = twovec_lev(&source, &target);
        assert_eq!(result, target.len());

        let source: String = "abc".into();
        let target: String = "".into();

        let result = twovec_lev(&source, &target);
        assert_eq!(result, source.len());
    }

    #[test]
    fn differencer_standard() {
        let differ = Differencer::standard();
        assert_eq!(differ.insert, 1.0);
        assert_eq!(differ.delete, 1.0);
        assert_eq!(differ.substitution, 1.0);
    }

    #[test]
    fn differencer_standard_abc_v_bca() {
        let differ = Differencer::standard();
        let left = "abc";
        let right = "bca";

        let result = differ.score(&left, &right);
        assert_eq!(result, 2.0);
    }
}
