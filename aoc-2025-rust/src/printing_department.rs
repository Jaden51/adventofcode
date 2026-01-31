use std::fs;

const ROLL: char = '@';
const DIRECTIONS: [[isize; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
    [1, 0],
    [1, -1],
    [0, -1],
];

pub fn printing_department(inputfile: &str) -> usize {
    println!("Optimizing forklifts!");

    let contents = fs::read_to_string(inputfile).expect("Error reading file");
    let map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = map.len();
    let cols = map[0].len();

    let mut result: usize = 0;

    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] == ROLL {
                let mut roll_count = 0;

                for d in DIRECTIONS {
                    let dir_row: isize = d[0];
                    let dir_col: isize = d[1];
                    let check_row = r as isize + dir_row;
                    let check_col = c as isize + dir_col;

                    if check_row < 0
                        || check_row >= rows as isize
                        || check_col < 0
                        || check_col >= cols as isize
                    {
                        continue;
                    }
                    if map[check_row as usize][check_col as usize] == ROLL {
                        roll_count += 1;
                    }
                }

                if roll_count < 4 {
                    result += 1;
                }
            }
        }
    }
    result
}
