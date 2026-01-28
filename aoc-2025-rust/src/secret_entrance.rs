use std::fs;

pub fn secret_entrance(filepath: &str) -> i32 {
    println!("Unlocking Secrent Entrance!");
    let content = fs::read_to_string(filepath).expect("Error reading file");

    let mut current: i32 = 50;
    let mut result: i32 = 0;

    for step in content.lines() {
        let direction = step.chars().next().expect("Empty line");
        let movement = &step[1..].parse().expect("Not a number");

        current = match direction {
            'L' => (current - movement).rem_euclid(100),
            'R' => (current + movement).rem_euclid(100),
            _ => panic!("Unknown direction: {direction}"),
        };

        if current == 0 {
            result += 1
        }
    }
    result
}
