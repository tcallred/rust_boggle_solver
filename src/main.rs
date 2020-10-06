use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Trie {
    prefix: String,
    is_word: bool,
    children: HashMap<char, Trie>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            prefix: "".to_string(),
            is_word: false,
            children: HashMap::new(),
        }
    }

    pub fn insert(&mut self, word: &str, prefix: String) {
        if word.len() > 0 {
            let ch = word.chars().nth(0).unwrap();
            let new_prefix = format!("{}{}", prefix, ch);
            if let Some(child) = self.children.get_mut(&ch) {
                child.insert(&word[1..], new_prefix.clone());
            } else {
                let mut child = Self {
                    prefix: new_prefix.clone(),
                    is_word: false,
                    children: HashMap::new(),
                };
                child.insert(&word[1..], new_prefix.clone());
                self.children.insert(ch, child);
            }
        } else {
            self.is_word = true;
        }
    }
}

#[derive(Clone, Debug, Copy)]
struct Coordinate {
    row: i64,
    col: i64,
}

impl Coordinate {
    // Returns a new coordinate if the move is valid. None otherwise.
    fn move_coord(&self, dy: i64, dx: i64, h: i64, w: i64) -> Option<Coordinate> {
        let new_row = self.row + dy;
        let new_col = self.col + dx;

        if new_row >= 0 && new_row < h && new_col >= 0 && new_col < w {
            Some(Coordinate {
                row: new_row,
                col: new_col,
            })
        } else {
            None
        }
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

fn boggle_solve(grid: &Vec<Vec<char>>, dict: &Trie) {
    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if let Some(next_trie) = dict.children.get(ch) {
                boggle_solve_help(
                    grid,
                    next_trie,
                    Coordinate {
                        row: i as i64,
                        col: j as i64,
                    },
                    vec![Coordinate {
                        row: i as i64,
                        col: j as i64,
                    }],
                )
            }
        }
    }
}

fn boggle_solve_help(grid: &Vec<Vec<char>>, dict: &Trie, c: Coordinate, path: Vec<Coordinate>) {
    if dict.is_word {
        println!("{}", dict.prefix);
    }

    // For every possible direction the path can take, if it is valid, take it.
    for a in -1..=1 {
        for b in -1..=1 {
            match c.move_coord(a, b, grid.len() as i64, grid[0].len() as i64) {
                Some(new_c) if !path.contains(&new_c) => {
                    if let Some(new_trie) = dict
                        .children
                        .get(&grid[new_c.row as usize][new_c.col as usize])
                    {
                        let mut new_path = path.clone();
                        new_path.push(new_c);

                        boggle_solve_help(grid, new_trie, new_c, new_path);
                    }
                }

                Some(_) => (),

                None => (),
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("words_alpha.txt").unwrap();
    // let dict: HashSet<String> = contents.split_whitespace().map(|s| s.to_string()).collect();
    let mut dict = Trie::new();

    for s in contents.split_whitespace() {
        dict.insert(s, "".to_string())
    }

    // println!("{:?}", dict);

    let grid: Vec<Vec<char>> = vec![
        vec!['c', 's', 't', 'e', 't'],
        vec!['a', 'i', 'r', 'l', 's'],
        vec!['p', 'd', 'a', 'e', 's'],
        vec!['u', 'e', 'c', 's', 'e'],
        vec!['r', 'o', 't', 'r', 'i'],
    ];

    boggle_solve(&grid, &dict)
}
