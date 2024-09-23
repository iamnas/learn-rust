

pub fn find_first_a(s:String) -> Option<u32> {

    for (index,c) in s.chars().enumerate() {
        if c == 'a' {
            return Some(index as u32);
        }
    }
        return None;

}
