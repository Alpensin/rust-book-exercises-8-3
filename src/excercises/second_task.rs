// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added,
// so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead
// (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
use std::collections::HashSet;

const HAY: &str = "hay";
const AY: &str = "ay";

pub fn pig_latin(word: &str, vowels: &HashSet<char>) -> String {
    let first_char = word.chars().next().unwrap();
    if vowels.contains(&first_char) {
        return format!("{word}-{HAY}");
    } else {
        let word_end = &word[1..];
        return format!("{word_end}-{first_char}{AY}");
    }
}
