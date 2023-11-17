use std::cmp::max;
use std::collections::VecDeque;

// see: https://en.wikipedia.org/wiki/Needleman%E2%80%93Wunsch_algorithm
//
//

#[derive(Eq, Debug, Clone)]
struct AlignmentNode {
    a_idx: usize,
    b_idx: usize,
    alignment: usize,
}

impl AlignmentNode {
    fn new(a_idx: usize, b_idx: usize, alignment: usize) -> Self {
        Self {
            a_idx,
            b_idx,
            alignment,
        }
    }
}

#[derive(Debug, Clone)]
struct Alignment {
    a: String,
    b: String,
}

impl Alignment {
    fn new() -> Self {
        Self {
            a: String::new(),
            b: String::new(),
        }
    }
}

pub struct SimpleScoringSystem {
    same: i32,
    diff: i32,
    indel: i32,
}

impl SimpleScoringSystem {
    pub fn new(same: i32, diff: i32, indel: i32) -> Self {
        Self { same, diff, indel }
    }
}

fn walk_back(
    ss: &SimpleScoringSystem,
    vecvec: &Vec<Vec<i32>>,
    str_a: &str,
    str_b: &str,
) -> (String, String) {
    let mut upper_align = String::new();
    let mut left_align = String::new();

    let mut jj = vecvec[0].len() - 1;
    let mut ii = vecvec.len() - 1;

    let mut csa = str_a.chars().rev();
    let mut csb = str_b.chars().rev();
    let mut todo: VecDeque<AlignmentNode> = VecDeque::new();
    let mut alignments: Vec<Alignment> = Vec::new();
    alignments.push(Alignment::new());
    todo.push_back(AlignmentNode::new(jj, ii, 0));

    loop {
        if let Some(cell) = todo.pop_front() {
            let AlignmentNode {
                a_idx: jj,
                b_idx: ii,
                alignment,
            } = cell;
            let cell_score = vecvec[ii][jj];
            println!("checking cell[{}][{}] : {:?}", ii, jj, cell_score);
            let up_left = vecvec[ii - 1][jj - 1];

            if ii > 0 && jj > 0 && (cell_score == up_left + ss.same || cell_score == up_left + ss.diff)
            {
                let (mut upper_align, mut left_align) = alignments[alignment];
                println!("  we have a match");
                upper_align.push(csa.next().unwrap());
                left_align.push(csb.next().unwrap());
                ii = ii - 1;
                jj = jj - 1;
            } else if ii > 0 && cell_score == vecvec[ii - 1][jj] + ss.indel {
                println!("  left indel");
                upper_align.push(csa.next().unwrap());
                left_align.push('-');
                ii = ii - 1;
            } else {
                println!("  upper indel");
                upper_align.push('-');
                left_align.push(csb.next().unwrap());
                jj = jj - 1;
            }
        } else {
            break;
        }
    }

    (
        upper_align.chars().rev().collect(),
        left_align.chars().rev().collect(),
    )
}
/*
*/

pub fn vecvec_nw(ss: &SimpleScoringSystem, src: &str, target: &str) -> i32 {
    let width = src.len();
    let height = target.len();

    let mut grid = vec![vec![0; width + 1]; height + 1];

    // base cases
    for (ii, etl) in grid[0].iter_mut().enumerate() {
        *etl = ii as i32 * ss.indel;
    }
    for (ii, row) in grid.iter_mut().enumerate() {
        row[0] = ii as i32 * ss.indel;
    }

    // fill in table
    for (ii, t_char) in target.chars().enumerate() {
        let row = ii + 1;
        //println!("row:{}", row);
        for (jj, s_char) in src.chars().enumerate() {
            let col = jj + 1;
            let match_cost = if t_char == s_char { ss.same } else { ss.diff };

            let indel_left_cost = grid[row][col - 1] + ss.indel;
            let indel_up_cost = grid[row - 1][col] + ss.indel;
            let match_cost = grid[row - 1][col - 1] + match_cost;
            let cell_cost = max(max(indel_left_cost, indel_up_cost), match_cost);
            grid[row][col] = cell_cost;
        }
    }

    for row in grid.iter() {
        println!("{:?}", row);
    }

    let (a_align, b_align) = walk_back(ss, &grid, src, target);
    println!("a_align:{:?}", a_align);
    println!("b_align:{:?}", b_align);

    // answer is final cell of grid
    grid[grid.len() - 1][grid[0].len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vecvec_nm_same() {
        let scoring_system = SimpleScoringSystem::new(1, -1, -1);

        let input: String = "abc".into();

        let result = vecvec_nw(&scoring_system, &input, &input);
        assert_eq!(result, input.len() as i32);
    }

    #[test]
    fn vecvec_nm_wikipedia_intro_example() {
        let scoring_system = SimpleScoringSystem::new(1, -1, -1);

        let left: String = "GCATGCG".into();
        let right: String = "GATTACA".into();

        let result = vecvec_nw(&scoring_system, &left, &right);
        assert_eq!(result, 0);
    }
}
