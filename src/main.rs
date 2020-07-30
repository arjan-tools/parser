use std::time::Instant;

fn main() {
    // (all the type annotations are superfluous)
    // A reference to a string allocated in read only memory
    let now = Instant::now();
    let pangram = String::from("the quick brown fox jumps over the lazy dog");
    println!("Pangram: {}", pangram);

    println!("{}", pangram.contains("quidk"));
    println!("{}", pangram.capacity());

    // Iterate over words in reverse, no new string is allocated
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    println!("{}", String::from_utf8(vec![240, 159, 146, 150]).unwrap());
    let new_now = Instant::now();
    println!("{:?}", new_now.duration_since(now));
    // Copy chars into a vector, sort and remove duplicates
    /* let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup(); */
}
