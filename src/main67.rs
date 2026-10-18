pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack_vec: Vec<char> = haystack.chars().collect();
    let needle_vec: Vec<char> = needle.chars().collect();
    for i in 0..haystack_vec.len() {
        let mut flag = false;
        for j in 0..needle_vec.len() {
            if i + j >= haystack_vec.len() || haystack_vec[i + j] != needle_vec[j] {
                flag = true;
                break;
            }
        }
        if !flag {
            return i as i32;
        }
    }
    -1
}
