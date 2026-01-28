pub mod secret_entrance;

fn main() {
    println!(
        "The secret entrance code is {}",
        secret_entrance::secret_entrance("inputs/day1.txt")
    );
}
