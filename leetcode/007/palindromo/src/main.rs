fn main() {
    impl Solution {
        pub fn is_palindrome(x: i32) -> bool {
            let n: String = x.to_string();
            let mut temp: Vec<char> = vec![];

            if !(x < 0) {
                for digito in n.chars() {
                    temp.insert(0, digito);
                }
                let x2: String = temp.into_iter().collect();
                let x2:i64 = x2.parse().unwrap();
                if (x as i64) == x2 {
                    return true
                }
                return false
            }
            false
        }
    }
}