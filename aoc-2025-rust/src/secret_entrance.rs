use std::fs;

pub fn secret_entrance(filepath: &str) -> i32 {
    println!("Unlocking Secrent Entrance!");
    println!("{filepath}");
    let contents = fs::read_to_string(filepath).expect("Error reading file");
    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

    let mut current: i32 = 50;
    let mut result: i32 = 0;

    for step in lines {
        let direction = &step[..1];
        let movement = &step[1..];
        let mut mov_int: i32 = movement.parse().expect("Not a number");
        mov_int %= 100;

        if direction == "L" {
            current -= mov_int;
            if current < 0 {
                current += 100;
            }
        } else if direction == "R" {
            current += mov_int;
            if current > 99 {
                current -= 100;
            }
        }

        if current == 0 {
            result += 1
        }
    }
    result
}
