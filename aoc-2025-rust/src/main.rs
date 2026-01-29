mod gift_shop;
mod secret_entrance;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args.first().unwrap());
        std::process::exit(1);
    }

    match args[1].as_str() {
        "1" => {
            let code = secret_entrance::secret_entrance("inputs/day1.txt");
            println!("The secret entrance code is {}!", code);
        }
        "2" => {
            let invalid_id_sum = gift_shop::giftshop("inputs/day2.txt");
            println!("The sum of invalid ids is {}!", invalid_id_sum);
        }
        _ => {
            eprintln!("Day not found {}", args[1]);
            std::process::exit(1);
        }
    }
}
