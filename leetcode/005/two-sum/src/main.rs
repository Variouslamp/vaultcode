use std::{collections::HashMap, vec};


fn main() {
    struct Solution {
        nums: Vec<i32>,
        target: i32,
    }
    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut memoria: HashMap<i32, i32> = HashMap::new();

            for (index1, numero) in nums.iter().enumerate() {
                let busca: i32 = target - *numero;
                let index1 = index1 as i32;

                if memoria.contains_key(&busca) {
                    return vec![memoria[&busca], index1];
                }

                memoria.insert(*numero, index1);
            }
            vec![]
        }
    }

    // Prueba la cual tiene que dar [3,4] para ser correcta
    let prueba = Solution::two_sum(vec![1, 3, 54, 6, 2, 1, 3, 6, 7], 8);
    print!("{:?}", prueba); // impresion de la prueba
}