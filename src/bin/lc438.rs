impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut target = vec![0; 26];
        let mut ret = vec![];
        p.chars()
            .for_each(|c| {
                target[(c as u8 - 'a' as u8) as usize] += 1;
            });

        let mut curr = vec![0; 26];
        s.chars()
            .take(p.len() - 1)
            .for_each(|c| {
                curr[(c as u8 - 'a' as u8) as usize] += 1;
            });
        let s = s.chars().collect::<Vec<_>>();
        let mut l = 0;
        let mut r = p.len() - 1;
        while r < s.len() {
            curr[(s[r] as u8 - 'a' as u8) as usize] += 1;
            if curr == target {
                ret.push(l as i32);
            }
            curr[(s[l] as u8 - 'a' as u8) as usize] -= 1;
            r += 1;
            l += 1;
        }
        ret
    }
}