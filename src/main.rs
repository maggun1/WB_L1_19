fn reverse_words(input: &str) -> String {
    let mut words: Vec<&str> = input.split_whitespace().collect();
    words.reverse();
    words.join(" ")
}

fn main() {
    let input = "snow dog sun";
    let reversed = reverse_words(&input);
    println!("{} - {}", input, reversed);
}