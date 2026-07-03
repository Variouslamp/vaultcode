use std::{collections::HashMap};

fn main() {
    struct Solution {} // Para que no me de error
    impl Solution {
        pub fn length_of_longest_substring(s: String) -> i32 {
            let mut index: i32 = 0;
            let mut memory: HashMap<char, i32> = HashMap::new();
            let mut longest_substring: i32 = 0;
            let mut temp_substring: i32;
            let mut pointer_t: i32 = 1;

            if s.len() == 1{
                return 1;
            }

        for letter in s.chars() {
                index += 1;
                
                if memory.contains_key(&letter) {
                    if !(memory[&letter] < pointer_t) {
                        pointer_t = memory[&letter] + 1;
                    }
                    memory.remove(&letter); 
                }
                memory.insert(letter, index);
                temp_substring = index - pointer_t + 1;
                if temp_substring > longest_substring {
                    longest_substring = temp_substring;
                }
                print!("indice: {}, puntero_t:{}, letra: {}, longest: {} \n", index, pointer_t, letter, longest_substring);
            }
            longest_substring
        }
    }

    let texto: String = String::from("au");
    let a = Solution::length_of_longest_substring(texto);
    print!("{}", a)
}
