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
            if check_invalid_id(&id_string) {
                result += id;
            }
        }
    }

    result
}

fn split_string_chunks(s: &str, i: usize) -> Vec<String> {
    s.chars()
        .collect::<Vec<_>>()
        .chunks(i)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

fn check_invalid_id(id: &str) -> bool {
    let id_len = id.len();

    for len in 1..id_len {
        let chunks = split_string_chunks(id, len);
        if all_chunks_equal(&chunks) {
            return true;
        }
    }
    false
}

fn all_chunks_equal(chunks: &[String]) -> bool {
    if let Some(first) = chunks.first() {
        chunks.iter().all(|chunk| chunk == first)
    } else {
        true
    }
}
