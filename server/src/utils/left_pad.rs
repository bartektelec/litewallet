pub fn left_pad<T: AsRef<str>>(string: T, fill: char, len: usize) -> String {
    let initial_ref = string.as_ref();
    let initial_string = initial_ref.to_string();

    let rev_initial: Vec<char> = initial_string.chars().rev().collect();

    let mut output: Vec<&char> = vec![];

    for n in 0..len {
        let char_found = rev_initial.get(n);

        match char_found {
            Some(c) => output.push(c),
            None => output.push(&fill),
        }
    }

    let result: String = output.into_iter().rev().collect();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn left_pad_should_truncate_if_short() {
        let input = "Hello there how are you";
        let output = " are you";

        let result = left_pad(input, 'X', 8);

        assert_eq!(output, result);
    }

    #[test]
    fn left_pad_should_return_same_if_length_is_same() {
        let input = "this is a string";

        let result = left_pad(input, 'X', input.len());

        assert_eq!(input, result);
    }

    #[test]
    fn left_pad_should_add_characters_to_left_side() {
        let input = "this is a string";
        let output = format!("XXX{}", input);

        let result = left_pad(input, 'X', input.len() + 3);

        assert_eq!(output, result);
    }
}
