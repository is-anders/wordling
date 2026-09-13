use reqwest::Client;

fn main() {
    // Get the word
    print!("What's the word?");
    let mut word = "".to_string();
    std::io::stdin()
        .read_line(&mut word)
        .expect("Failed to readline");
    let word = word.trim();

    // Check the word
    if word.is_empty() || !word.chars().all(char::is_alphabetic) {
        println!("Word is empty or contains unexpected chars");
    }

    // Search the wordA
    let client = Client::new();
    let response = client
        .get("https://api.dictionaryapi.dev/api/v2/entries/en/word")
        .send()
        .unwrap();
}
