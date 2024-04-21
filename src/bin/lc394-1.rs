struct Solution;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let word = word.chars().collect::<Vec<_>>();
        let mut lowercases = vec![0; 26];
        let mut uppercases = vec![0; 26];
        let mut marked = vec![0; 26];
        let mut res = 0;
        for ch in word {
            let mut idx = 0;
            if ch as u8 >= 'a' as u8 && ch as u8 <= 'z' as u8 {
                idx = (ch as u8 - 'a' as u8) as usize;
                if marked[idx] > 0 {
                    marked[idx] = -1;
                    continue;
                }
                lowercases[idx] += 1;
            } else {
                idx = (ch as u8 - 'A' as u8) as usize;
                if lowercases[idx] > 0 && marked[idx] >= 0 {
                    marked[idx] = 1;
                    uppercases[idx] += 1;
                } else {
                    marked[idx] = -1;
                }
            }
        }

        for v in marked {
            if v > 0 {
                res += 1;
            }
        }
        res
    }
}

fn main() {
    todo!();
}
