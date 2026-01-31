use std::fs;

const OPEN_SPACE: char = '.';
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
    let mut result: usize = 0;
    let mut map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    loop {
        let (rolls_removed, new_map) = remove_rolls(map);
        if rolls_removed == 0 {
            break;
        }

        result += rolls_removed;
        map = new_map;
    }
    result
}

pub fn remove_rolls(mut map: Vec<Vec<char>>) -> (usize, Vec<Vec<char>>) {
    let rows = map.len();
    let cols = map[0].len();
    let mut rolls_removed: usize = 0;
    let mut to_remove: Vec<(usize, usize)> = Vec::new();

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
                    rolls_removed += 1;
                    to_remove.push((r, c));
                }
            }
        }
    }
    for (r, c) in to_remove {
        map[r][c] = OPEN_SPACE;
    }
    (rolls_removed, map)
}
