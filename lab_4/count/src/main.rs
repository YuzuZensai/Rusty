fn main() {
    println!("{:?}", count_negative(&[1, 2, -3, 4, -6, 7]));
    println!("{:?}", count_negative_ordinary_loop(&[1, 2, -3, 4, -6, 7]));
    println!("{:?}", doubles(&[1, 2, 3]));
    println!("{:?}", doubles_ordinary_loop(&[1, 2, 3]));
    println!("{:?}", doubles_recursion(&[1, 2, 3]));
}

fn count_negative(v: &[i64]) -> usize {
    // Iterate over the vector, filter out the negative numbers, and count them up
    v.iter()
        .filter(|&&x| x < 0)
        .count()
}

fn count_negative_ordinary_loop(v: &[i64]) -> usize {
    // Initialize a counter
    let mut count = 0;

    // Iterate over the vector
    for x in v {
        // If the number is negative, increment the counter
        // The * is needed to dereference the reference to the number
        if *x < 0 {
            // Increment the counter
            count += 1;
        }
    }
    // Return the counter
    count
}

#[test]
fn test_counting() {
    assert_eq!(count_negative(&[]), 0);
    assert_eq!(count_negative(&[1, 2, -3, 4, -6, 7]), 2);
}

#[test]
fn test_counting_ordinary_loop() {
    assert_eq!(count_negative_ordinary_loop(&[]), 0);
    assert_eq!(count_negative_ordinary_loop(&[1, 2, -3, 4, -6, 7]), 2);
}

fn doubles(v: &[i64]) -> Vec<i64> {
    // Iterate over the vector, multiply each number by 2, and collect the results into a new vector
    v.iter()
        .map(|x| x * 2)
        .collect()
}

fn doubles_ordinary_loop(v: &[i64]) -> Vec<i64> {
    // Initialize a vector to hold the results
    let mut result = Vec::new();
    // Iterate over the vector
    for x in v {
        // Multiply each number by 2 and push the result onto the vector
        result.push(x * 2);
    }
    // Return the vector
    result
}

fn doubles_recursion(v: &[i64]) -> Vec<i64> {
    // If the vector is empty, return an empty vector
    if v.is_empty() {
        return Vec::new();
    }
    // Otherwise, multiply the first number by 2 and concatenate it with the rest of the vector
    let mut result = vec![v[0] * 2];
    result.extend_from_slice(&doubles_recursion(&v[1..]));
    result
}

#[test]
fn test_doubles() {
    assert_eq!(doubles(&[]), []);
    assert_eq!(doubles(&[1, 2, 3]), [2, 4, 6]);
    assert_eq!(doubles(&[1, -2, 3, 4, 5]), [2, -4, 6, 8, 10]);
}

#[test]
fn test_doubles_ordinary_loop() {
    assert_eq!(doubles_ordinary_loop(&[]), []);
    assert_eq!(doubles_ordinary_loop(&[1, 2, 3]), [2, 4, 6]);
    assert_eq!(doubles_ordinary_loop(&[1, -2, 3, 4, 5]), [2, -4, 6, 8, 10]);
}

#[test]
fn test_doubles_recursion() {
    assert_eq!(doubles_recursion(&[]), []);
    assert_eq!(doubles_recursion(&[1, 2, 3]), [2, 4, 6]);
    assert_eq!(doubles_recursion(&[1, -2, 3, 4, 5]), [2, -4, 6, 8, 10]);
}
