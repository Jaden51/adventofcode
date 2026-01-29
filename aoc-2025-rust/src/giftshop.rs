use std::fs;

pub fn giftshop(inputfile: &str) -> u64 {
    println!("Checking gift shop invalid product ids");

    let content = fs::read_to_string(inputfile).expect("Error reading file");

    let mut result = 0;

    for id_range in content.trim().split(",") {
        let mut bounds = id_range.split("-");
        let lower_bound: u64 = bounds.next().unwrap().trim().parse::<u64>().unwrap();
        let upper_bound: u64 = bounds.next().unwrap().trim().parse::<u64>().unwrap();

        for id in lower_bound..(upper_bound + 1) {
            let id_string = id.to_string();
            if id_string.len() % 2 == 1 {
                continue;
            }

            let mid = id_string.len() / 2;
            let (left, right) = id_string.split_at(mid);
            if left == right {
                result += id;
            }
        }
    }

    result
}
