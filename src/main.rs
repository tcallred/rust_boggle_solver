use std::collections::HashSet;
use std::fs;

const MAX_WORD_LEN: usize = 12;


#[derive(Clone, Debug, Copy)]
struct Coordinate {
    row: i64,
    col: i64
}


impl Coordinate {
    // Returns a new coordinate if the move is valid. None otherwise.
    fn move_c(&self, a: i64, b: i64, h: i64, w: i64) -> Option<Coordinate> {
        let h_range = 0..h;
        let w_range = 0..w;
        if h_range.contains(&(self.row + a)) && w_range.contains(&(self.col + b)) {
            Some(Coordinate {row: self.row + a, col: self.col + b}) 
        } 
        else {
            None
        } 
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    } 
}


fn boggle_solve(grid: &Vec<Vec<char>>, dict: &HashSet<String>) {
    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            boggle_solve_help(grid, dict, ch.to_string(), Coordinate{row: i as i64, col: j as i64}, vec![Coordinate{row: i as i64, col: j as i64}])
        }
    }

}


fn boggle_solve_help(grid: &Vec<Vec<char>>, dict: &HashSet<String>, word: String, c: Coordinate, path: Vec<Coordinate>) {
    // Base case.
    // No more work if the string is long enough
    if word.len() > MAX_WORD_LEN {
        return;
    }

    if word.len() > 3 && dict.contains(&word) {
        println!("{}", word);
    }

    // For every possible direction the path can take, if it is valid, take it.
    for a in -1..=1 {
        for b in -1..=1 {
           match c.move_c(a, b, grid.len() as i64, grid[0].len() as i64) {

                Some(new_c) if !path.contains(&new_c) => {
                    let mut new_path = path.clone();
                    new_path.push(new_c);
                    let new_word = format!("{}{}", &word, &grid[new_c.row as usize][new_c.col as usize]);
                    
                    boggle_solve_help(grid, dict, new_word, new_c, new_path);
                },

                Some(_) => (),

                None => ()

           } 
        }
    } 
}

fn main() {
    let contents = fs::read_to_string("words_alpha.txt").unwrap();
    let dict: HashSet<String> = contents.split_whitespace().map(|s| s.to_string()).collect();

    let grid: Vec<Vec<char>> = vec![
        vec!['c', 's', 't', 'e', 't'],
        vec!['a', 'i', 'r', 'l', 's'],
        vec!['p', 'd', 'a', 'e', 's'],
        vec!['u', 'e', 'c', 's', 'e'],
        vec!['r', 'o', 't', 'r', 'i']
    ];

    boggle_solve(&grid, &dict)

}