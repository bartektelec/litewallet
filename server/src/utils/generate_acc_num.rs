const PREFIX: usize = 133769;
const MAX_LEN: usize = 11;

use crate::utils::left_pad::left_pad;
use rand::Rng;

pub fn generate_acc_num() -> u64 {
    // generate a random 11 -char long number

    let prefix_len = PREFIX.to_string().len();
    let suffix_len = MAX_LEN - prefix_len;

    let mut rng = rand::thread_rng();

    let suf_raw = rng.gen_range(0..suffix_len);

    let suf_formatted = left_pad(suf_raw.to_string(), '0', suffix_len);

    let whole_num = format!("{}{}", PREFIX, suf_formatted);

    let result = whole_num.parse::<u64>().unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_right_length() {
        for _ in 0..100 {
            let output = generate_acc_num();
            assert_eq!(output.to_string().len(), MAX_LEN);
        }
    }
}
