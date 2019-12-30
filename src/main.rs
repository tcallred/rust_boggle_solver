const MAX_WORD_LEN: usize = 3;

#[derive(Clone, Debug, Copy)]
struct Coordinate {
    row: i64,
    col: i64
}


impl Coordinate {
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


fn boggle_solve(grid: &Vec<Vec<char>>) {

    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            boggle_solve_help(grid, ch.to_string(), Coordinate{row: i as i64, col: j as i64}, vec![Coordinate{row: i as i64, col: j as i64}])
        }
    }

}


fn boggle_solve_help(grid: &Vec<Vec<char>>, word: String, c: Coordinate, path: Vec<Coordinate>) {

    // Base case.
    // No more work if the string is long enough
    if word.len() > MAX_WORD_LEN {
        return;
    }

    println!("{}", word);

    for a in -1..=1 {
        for b in -1..=1 {
           match c.move_c(a, b, grid.len() as i64, grid[0].len() as i64) {

                Some(new_c) if !path.contains(&new_c) => {
                    let mut new_path = path.clone();
                    new_path.push(new_c);
                    let new_word = format!("{}{}", &word, &grid[new_c.row as usize][new_c.col as usize]);
                    
                    boggle_solve_help(grid, new_word, new_c, new_path);
                },

                Some(_) => (),

                None => ()

           } 
        }
    } 
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn three_by_three() {
        let grid = vec![
            vec!['A', 'B', 'C'],
            vec!['D', 'E', 'F'],
            vec!['G', 'H', 'I']
        ];

        boggle_solve(&grid);
    }
}