struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let s = s.as_bytes();
        let mut concatable = vec![false; s.len() + 1];
        concatable[0] = true;

        for idx in 1..=s.len() {
            for word in &word_dict {
                let word = word.as_bytes();
                if word.len() > idx {
                    continue;
                }
                let prev_idx = idx - word.len();
                if concatable[prev_idx] && s[prev_idx..idx] == *word {
                    concatable[idx] = true;
                    break;
                }
            }
        }
        concatable[s.len()]
    }
}
