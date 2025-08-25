
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut s_count = [0; 26];
    let mut t_count = [0; 26];
    for sc in s.chars() {
        s_count[(sc as u8 - 'a' as u8) as usize] += 1;
    }
    for tc in t.chars() {
        t_count[(tc as u8 - 'a' as u8) as usize] += 1;  
    }
    return s_count == t_count;
}
