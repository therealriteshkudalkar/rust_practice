
// Strategy: Naive
//  Create all possible sub strings and check if the substring is palindrome
//  while keeping the track of the largest one

// Strategy: Optimized I
//  Create a map: len > index_range > bool
//  Loop through substrings of different lengths

// Strategy: Optimized II
//  Create a vector of map of string to bool
//  Loop through the substrings of different lengths and use the previously computed results

// Strategy: Optimized III
//  Create a vector of a vector of bools
//  Loop through the substrings of different lengths and use the previously computed results

pub fn is_palindrome(s: &str) -> bool {
    let char_vec: Vec<char> = s.chars().collect();
    let char_vec_len = char_vec.len();
    for i in 0..char_vec_len {
        if char_vec[i] != char_vec[char_vec_len - i - 1] {
            return false
        }
    }
    true
}

#[allow(dead_code)]
pub fn longest_palindrome(s: String) -> String {
    let str_len = s.len();
    let mut longest_palindrome = String::from("");
    for i in 0..str_len {
        for j in i..str_len {
            // Add another loop that check if character from j and i and equal
            if is_palindrome(&s[i..j+1]) && j - i + 1 > longest_palindrome.len() {
                longest_palindrome = String::from(&s[i..j+1])
            }
        }
    }
    longest_palindrome
}

pub fn longest_palindrome_optimized(s: String) -> String {
    let mut start_index = 0;
    let mut end_index = 0;
    let str_chars = s.chars().collect::<Vec<char>>();
    let str_len = str_chars.len();
    let mut vec_of_vec: Vec<Vec<bool>> = vec![vec![false; str_len + 1]; str_len + 1];
    for l in 1..=str_len {
        // let mut internal_map = Vec::new();
        for i in 0..=str_len - l {
            // check if the string from i to j (inclusive) is palindromic or not
            if l == 1 {
                if l > end_index - start_index {
                    start_index = i;
                    end_index = i + l;
                }
                vec_of_vec[i][i + l - 1] = true;
            } else if l == 2 {
                let val = str_chars[i] == str_chars[i + l - 1];
                if val && l > end_index - start_index {
                    start_index = i;
                    end_index = i + l;
                }
                vec_of_vec[i][i + l - 1] = val;
            } else {
                // For other cases check if i + 1 to j - 1
                if str_chars[i] == str_chars[i + l - 1] {
                    // Check if substring is palindrome
                    let is_substr_palindrome = vec_of_vec[i + 1][i + l - 2];
                    if is_substr_palindrome && l > end_index - start_index {
                        start_index = i;
                        end_index = i + l;
                    }
                    vec_of_vec[i][i + l - 1] = is_substr_palindrome;
                } else {
                    vec_of_vec[i][i + l - 1] = false;
                }
            }
        }
    }
    String::from(&s[start_index..end_index])
}

pub fn main66() {
    let str = String::from("aacabdkacaa");
    println!("Input:{}; Longest palindrome: {}", &str, longest_palindrome_optimized(str.clone()));
    
    let str = String::from("a");
    println!("Input:{}; Longest palindrome: {}", &str, longest_palindrome_optimized(str.clone()));
    
    let str = String::from("ac");
    println!("Input:{}; Longest palindrome: {}", &str, longest_palindrome_optimized(str.clone()));
    
    let str = String::from("babad");
    println!("Input:{}; Longest palindrome: {}", &str, longest_palindrome_optimized(str.clone()));
    
    let str = String::from("cbbd");
    println!("Input:{}; Longest palindrome: {}", &str, longest_palindrome_optimized(str.clone()));
    
    let str = String::from("boqylncwfahjzvawrojyhqiymirlkfzkhtvmbjnbfjxzewqqqcfnximdnrxtrbafkimcqvuprgrjetrecqkltforcudmbpofcxqdcirnaciggflvsialdjtjnbrayeguklcbdbkouodxbmhgtaonzqftkebopghypjzfyqutytbcfejhddcrinopynrprohpbllxvhitazsjeyymkqkwuzfenhphqfzlnhenldbigzmriikqkgzvszztmvylzhbfjoksyvfdkvshjzdleeylqwsapapduxrfbwskpnhvmagkolzlhakvfbvcewvdihqceecqhidvwecvbfvkahlzlokgamvhnpkswbfrxudpapaswqlyeeldzjhsvkdfvyskojfbhzlyvmtzzsvzgkqkiirmzgibdlnehnlzfqhphnefzuwkqkmyyejszatihvxllbphorprnyponircddhjefcbtytuqyfzjpyhgpobektfqznoatghmbxdouokbdbclkugeyarbnjtjdlaisvlfggicanricdqxcfopbmducroftlkqcertejrgrpuvqcmikfabrtxrndmixnfcqqqwezxjfbnjbmvthkzfklrimyiqhyjorwavzjhafwcnlyqob");
    println!("Input:{}; Longest palindrome: {}", &str, longest_palindrome_optimized(str.clone()));
    
    let str = String::from("ajgiljtperkvubjmdsefcylksrxtftqrehoitdgdtttswwttmfuvwgwrruuqmxttzsbmuhgfaoueumvbhajqsaxkkihjwevzzedizmrsmpxqavyryklbotwzngxscvyuqjkkaotitddlhhnutmotupwuwyltebtsdfssbwayuxrbgihmtphshdslktvsjadaykyjivbzhwujcdvzdxxfiixnzrmusqvwujjmxhbqbdpauacnzojnzxxgrkmupadfcsujkcwajsgintahwgbjnvjqubcxajdyyapposrkpqtpqfjcvbhlmwfutgognqxgaukpmdyaxghgoqkqnigcllachmwzrazwhpppmsodvxilrccfqgpkmdqhoorxpyjsrtbeeidsinpeyxxpsjnymxkouskyhenzgieybwkgzrhhrzgkwbyeigznehyksuokxmynjspxxyepnisdieebtrsjypxroohqdmkpgqfccrlixvdosmppphwzarzwmhcallcginqkqoghgxaydmpkuagxqngogtufwmlhbvcjfqptqpkrsoppayydjaxcbuqjvnjbgwhatnigsjawckjuscfdapumkrgxxznjozncauapdbqbhxmjjuwvqsumrznxiifxxdzvdcjuwhzbvijykyadajsvtklsdhshptmhigbrxuyawbssfdstbetlywuwputomtunhhlddtitoakkjquyvcsxgnzwtoblkyryvaqxpmsrmzidezzvewjhikkxasqjahbvmueuoafghumbszttxmquurrwgwvufmttwwstttdgdtioherqtftxrsklycfesdmjbuvkreptjligja");
    println!("Input:{}; Longest palindrome: {}", &str, longest_palindrome_optimized(str.clone()));
    
    let str = String::from("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    println!("Input:{}; Longest palindrome: {}", &str, longest_palindrome_optimized(str.clone()));
}