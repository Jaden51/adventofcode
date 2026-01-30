use std::fs;
pub fn lobby(inputfile: &str) -> usize {
    println!("Starting lobby staircase!");

    let contents = fs::read_to_string(inputfile).expect("Error reading file");
    let mut result: usize = 0;
    for line in contents.lines() {
        let mut stack: Vec<u32> = Vec::new();
        let digits: Vec<u32> = line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(0))
            .collect();

        let n = digits.len();
        let to_remove = n - 12;
        let mut removed = 0;

        for &int in &digits {
            while let Some(&top) = stack.last() {
                if int > top && removed < to_remove {
                    stack.pop();
                    removed += 1;
                } else {
                    break;
                }
            }
            stack.push(int);
        }

        let jolts: String = stack.iter().take(12).map(|d| d.to_string()).collect();

        result += jolts.parse::<usize>().unwrap();
    }
    result
}
