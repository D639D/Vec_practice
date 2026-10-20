use std::collections::HashMap;

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("{:?}", median(&mut numbers));
    println!("{:?}", mode_of_list(&numbers));
    println!("{:?}", average_value(numbers));
}

fn median(numbers: &mut Vec<u32>) -> Option<u32> {
    if numbers.is_empty() {
        None
    } else {
        numbers.sort();
        Some(numbers[numbers.len() / 2])
    }
}

fn mode_of_list(numbers: &[u32]) -> Option<u32> {
    if numbers.is_empty() {
        None
    } else {
        let mut counts = HashMap::new();
        for item in numbers {
            let count = counts.entry(item).or_insert(0);
            *count += 1;
        }
        let mut max_count = 0;
        let mut mode = 0;
        for (key, value) in counts {
            if max_count < value {
                max_count = value;
                mode = *key;
            }
        }
        Some(mode)
    }
}

fn average_value(numbers: Vec<u32>) -> Option<u32> {
    if numbers.is_empty() {
        None
    } else {
        let mut tmp = 0;
        let mut count = 0;
        for num in numbers {
            tmp += num;
            count += 1;
        }
        Some(tmp / count)
    }
}
