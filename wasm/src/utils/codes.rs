// Removed i, I, o, O -> 48 chars
// !! MUST be sorted by ascii values
static CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghjklmnpqrstuvwxyz";

pub fn code_to_string(mut code: u64) -> Option<String> {
    let mut string = String::new();

    while code != 0 {
        let rem = code % CHARS.len() as u64;
        string.insert(0, char::from(CHARS[rem as usize]));
        code /= CHARS.len() as u64;
    }

    Some(string)
}

pub fn string_to_code(string: &str) -> Option<u64> {
    let mut code = 0u64;

    for char in string.chars() {
        let index = CHARS.iter().position(|c| char::from(*c) == char)?;
        code = code * (CHARS.len() as u64) + (index as u64);
    }
    Some(code)
}
