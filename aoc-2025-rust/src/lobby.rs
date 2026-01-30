use std::fs;

pub fn lobby(inputfile: &str) -> usize {
    println!("Starting lobby staircase!");
    let contents = fs::read_to_string(inputfile).expect("Error reading file");

    let mut result: usize = 0;

    for line in contents.lines() {
        let mut stack: Vec<u32> = Vec::new();
        let mut char1 = String::new();
        let mut jolts = String::new();

        for c in line.trim().chars() {
            let int = c.to_digit(10).unwrap_or(0);
            if stack.is_empty() {
                stack.push(int);
                continue;
            }
            while let Some(&top) = stack.last() {
                if int > top {
                    if stack.len() == 1 {
                        char1 = top.to_string();
                    }
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(int);
        }

        if stack.len() == 1 {
            jolts.push_str(&char1);
            jolts.push_str(&stack.first().unwrap().to_string());
        } else {
            jolts.push_str(&stack.first().unwrap().to_string());
            jolts.push_str(&stack.get(1).unwrap().to_string());
        }
        result += jolts.parse::<usize>().unwrap();
    }
    result
}
