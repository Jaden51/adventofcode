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
        let rolls_removed = remove_rolls(&mut map);
        if rolls_removed == 0 {
            break;
        }

        result += rolls_removed;
    }
    result
}

pub fn remove_rolls(map: &mut [Vec<char>]) -> usize {
    let rows = map.len();
    let cols = map[0].len();
    let mut rolls_removed: usize = 0;
    let mut to_remove: Vec<(usize, usize)> = Vec::new();

    for r in 0..rows {
        for c in 0..cols {
            if map[r][c] == ROLL {
                let mut roll_count = 0;

                for [dir_row, dir_col] in DIRECTIONS {
                    let Some(check_row) = r.checked_add_signed(dir_row) else {
                        continue;
                    };
                    let Some(check_col) = c.checked_add_signed(dir_col) else {
                        continue;
                    };
                    let Some(row) = map.get(check_row) else {
                        continue;
                    };
                    let Some(&cell) = row.get(check_col) else {
                        continue;
                    };
                    if cell == ROLL {
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
    rolls_removed
}
