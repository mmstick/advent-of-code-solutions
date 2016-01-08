fn main() {
    let mut input = String::from("1113222113");
    for _ in 0..50 { input = calculate(&input); }
    println!("{}", input.len());
}
fn calculate(current_input: &str) -> String {
    let mut output       = String::new();
    let mut iterator     = current_input.chars();
    let mut previous     = iterator.next().unwrap();
    let mut count: usize = 1;
    for (id, character) in iterator.enumerate() {
        if character != previous {
            output.push_str(&count.to_string());
            output.push(previous);
            count = 0;
            previous = character;
        }
        count += 1;
        if id == current_input.len()-2 && previous == character {
            output.push_str(&count.to_string());
            output.push(previous);
        }
    }
    return output;
}

#[test]
fn test_once() {
    let test = "33221";
    let output = calculate(test);
    assert_eq!(output, "232211");
}

#[test]
fn test_looped() {
    let test = "33221";
    let mut output = String::from(test);
    for _ in 0..2 {
        output = calculate(&output);
    }
    assert_eq!(&output, "12132221");
}
