fn main() {
    println!("{:?}", quote("abcd", '*'));
    println!("{:?}", quote_list(&["abcd", "xyz"], '*'));
    println!("{:?}", quote_list_ordinary_loop(&["abcd", "xyz"], '*'));
    println!("{:?}", quote_list_recursion(&["abcd", "xyz"], '*'));
}

fn quote(s: &str, c: char) -> String {
    // Quote the string
    format!("{}{}{}", c, s, c)
}

fn quote_list(v: &[&str], c: char) -> Vec<String> {
    // Iterate over the vector, quote each string, and collect the results into a new vector
    v.iter()
        .map(|s| format!("{}{}{}", c, s, c))
        .collect()
}

fn quote_list_ordinary_loop(v: &[&str], c: char) -> Vec<String> {
    // Initialize a vector to hold the results
    let mut result = Vec::new();

    // Iterate over the vector
    for s in v {
        // Quote each string and push the result onto the vector
        result.push(format!("{}{}{}", c, s, c));
    }

    // Return the vector
    result
}

fn quote_list_recursion(v: &[&str], c: char) -> Vec<String> {
    // Base case: if the vector is empty, return an empty vector
    if v.is_empty() {
        Vec::new()
    } else {
        // Quote the first element and append the result of quoting the rest of the vector
        let mut result = quote_list_recursion(&v[1..], c);
        // Prepend the quoted first element to the result
        result.insert(0, format!("{}{}{}", c, v[0], c));
        // Return the result
        result
    }
}

#[test]
fn test_quotes() {
    assert_eq!(quote("", '*'), "**");
    assert_eq!(quote("abcd", '*'), "*abcd*");
    assert_eq!(quote("xyz", '*'), "*xyz*");
}

#[test]
fn test_quotes_list() {
    assert_eq!(quote_list(&[] as &[&str], '*'), &[] as &[&str]);
    assert_eq!(quote_list(&["abcd", "xyz"], '*'), ["*abcd*", "*xyz*"]);
    assert_eq!(quote_list(&["abcd", "xyz", "abc"], '*'), ["*abcd*", "*xyz*", "*abc*"]);
}

#[test]
fn test_quotes_ordinary_loop() {
    assert_eq!(quote_list_ordinary_loop(&[] as &[&str], '*'), &[] as &[&str]);
    assert_eq!(quote_list_ordinary_loop(&["abcd", "xyz"], '*'), ["*abcd*", "*xyz*"]);
    assert_eq!(quote_list_ordinary_loop(&["abcd", "xyz", "abc"], '*'), [
        "*abcd*",
        "*xyz*",
        "*abc*",
    ]);
}

#[test]
fn test_quotes_recursion() {
    assert_eq!(quote_list_recursion(&[] as &[&str], '*'), &[] as &[&str]);
    assert_eq!(quote_list_recursion(&["abcd", "xyz"], '*'), ["*abcd*", "*xyz*"]);
    assert_eq!(quote_list_recursion(&["abcd", "xyz", "abc"], '*'), ["*abcd*", "*xyz*", "*abc*"]);
}
