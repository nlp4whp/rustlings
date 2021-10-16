// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// As always, there are hints if you execute `rustlings hint iterators2`!

/* Reference:
    https://kaisery.github.io/trpl-zh-cn/ch13-02-iterators.html
    https://stackoverflow.com/questions/56033289/join-iterator-of-str
    https://www.reddit.com/r/rust/comments/6q4uqc/help_whats_the_best_way_to_join_an_iterator_of/
*/


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    // `iter.collect()`会消费迭代器
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + &c.collect::<String>()
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // vec![]
    // words.iter().map(|x| x.to_string()).collect::<Vec<String>>()
    words.iter().map(|x| capitalize_first(x)).collect::<Vec<String>>()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    // String::new()
    words.iter().map(|x| capitalize_first(x)).collect::<String>()
    // words.iter().map(|x| capitalize_first(x)).collect::<Vec<String>>().join("")  // also correct

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
