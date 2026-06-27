fn main () {
    struct Solution {
        nums: Vec<i32>
    }
    impl Solution {
        pub fn pivot_index(nums: Vec<i32>) -> i32 {
            let suma_total: i32 = nums.iter().sum();
            let mut suma_izquierda = 0;
            for (i, &valor) in nums.iter().enumerate() {
                if suma_izquierda == suma_total - suma_izquierda - valor {
                    return i as i32;
                }
                suma_izquierda += valor;
            }
            -1
        }
    }
}