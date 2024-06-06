fn first_occurrence(array: &[i32], value: i32) -> Option<usize> {
    array.iter().position(|&x| x == value)
}
fn is_prime(number: u32) -> bool {
    if number <= 1 {
        return false;
    }
    for i in 2..=((number as f64).sqrt() as u32) {
        if number % i == 0 {
            return false;
        }
    }
    true
}

fn shortest_word(sentence: &str) -> &str {
    sentence
        .split_whitespace()
        .min_by_key(|word| word.len())
        .unwrap_or("")
}
fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let reversed_chars: Vec<char> = chars.iter().rev().cloned().collect();
    chars == reversed_chars
}

fn find_median(numbers: &[i32]) -> f64 {
    let len = numbers.len();
    if len == 0 {
        return 0.0;
    }
    if len % 2 == 0 {
        (numbers[len / 2 - 1] as f64 + numbers[len / 2] as f64) / 2.0
    } else {
        numbers[len / 2] as f64
    }
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return "".to_string();
    }
    let mut prefix = strings[0].to_string();
    for string in strings.iter().skip(1) {
        while !string.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return "".to_string();
            }
        }
    }
    prefix
}

fn kth_smallest(array: &mut [i32], k: usize) -> i32 {
    array.sort();
    array[k - 1]
}

#[derive(Debug, Clone)]
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn max_depth(root: Option<Box<Node>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            std::cmp::max(left_depth, right_depth) + 1
        }
        None => 0,
    }
}

fn merge_sorted_arrays(array1: &[i32], array2: &[i32]) -> Vec<i32> {
    let mut merged = vec![];
    let mut i = 0;
    let mut j = 0;

    while i < array1.len() && j < array2.len() {
        if array1[i] <= array2[j] {
            merged.push(array1[i]);
            i += 1;
        } else {
            merged.push(array2[j]);
            j += 1;
        }
    }

    while i < array1.len() {
        merged.push(array1[i]);
        i += 1;
    }

    while j < array2.len() {
        merged.push(array2[j]);
        j += 1;
    }

    merged
}

fn max_sub_array(numbers: &[i32]) -> i32 {
    let mut max_sum = numbers[0];
    let mut current_sum = numbers[0];

    for &num in &numbers[1..] {
        current_sum = current_sum.max(num);
        current_sum = current_sum + num;
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}

fn main() {
    // is_palindrome
    let test_palindrome = "madam";
    println!(
        "Is '{}' a palindrome? {}",
        test_palindrome,
        is_palindrome(test_palindrome)
    );

    // first_occurrence
    let numbers = [5, 7, 9, 11, 13, 15];
    let search_value = 11;
    println!(
        "First occurrence of {} in {:?} is at index {:?}",
        search_value,
        numbers,
        first_occurrence(&numbers, search_value)
    );

    // shortest_word
    let phrase = "The quick brown fox jumps over the lazy dog";
    println!("Shortest word in the phrase is '{}'", shortest_word(phrase));

    // is_prime
    let prime_candidate = 31;
    println!(
        "Is {} a prime number? {}",
        prime_candidate,
        is_prime(prime_candidate)
    );

    // find_median
    let sorted_numbers = [1, 3, 5, 7, 9, 11];
    println!(
        "Median of {:?} is {}",
        sorted_numbers,
        find_median(&sorted_numbers)
    );

    // longest_common_prefix
    let word_list = ["interstellar", "internet", "interval"];
    println!(
        "Longest common prefix of {:?} is '{}'",
        word_list,
        longest_common_prefix(&word_list)
    );

    // kth_smallest
    let mut unsorted_numbers = [7, 2, 1, 6, 8, 5, 3, 4];
    let k = 4;

    println!("Before sorting: {:?}", unsorted_numbers); // Immutable borrow
    let unsorted_numbers_mut = &mut unsorted_numbers; // Mutable borrow
    let kth_element = kth_smallest(unsorted_numbers_mut, k);
    println!(
        "The {}th smallest element in {:?} is {}",
        k, unsorted_numbers_mut, kth_element
    );

    // max_depth
    let root = Some(Box::new(Node {
        val: 10,
        left: Some(Box::new(Node {
            val: 20,
            left: Some(Box::new(Node {
                val: 40,
                left: None,
                right: None,
            })),
            right: Some(Box::new(Node {
                val: 50,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(Node {
            val: 30,
            left: None,
            right: Some(Box::new(Node {
                val: 60,
                left: None,
                right: None,
            })),
        })),
    }));
    println!("Maximum depth of the tree is {}", max_depth(root));

    // reverse_string
    let string_to_reverse = "Rustacean";
    println!(
        "The reverse of '{}' is '{}'",
        string_to_reverse,
        reverse_string(string_to_reverse)
    );

    // merge_sorted_arrays
    let array1 = [1, 4, 7];
    let array2 = [2, 5, 8];
    println!(
        "Merged sorted arrays: {:?}",
        merge_sorted_arrays(&array1, &array2)
    );

    // max_sub_array
    let number_sequence = [2, -1, 2, 3, 4, -5];
    println!(
        "Maximum subarray sum is {}",
        max_sub_array(&number_sequence)
    );
}
