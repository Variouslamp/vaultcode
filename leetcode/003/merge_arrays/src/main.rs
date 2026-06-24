use std::vec;

fn main() {
    struct Solution {
        nums1: Vec<Vec<i32>>,
        nums2: Vec<Vec<i32>>
    }

    impl Solution {
        fn return_all (
            vect1: &Vec<Vec<i32>>,
            vect2: &Vec<Vec<i32>>, 
            contador1: usize, 
            contador2: usize, 
            order_list: &mut Vec<Vec<i32>>){

            if vect1.len() >= contador1{
                for vector in contador2..vect2.len(){
                    order_list.push(vect2[vector].clone());
                }
            }
            if vect2.len() >= contador2{
                for vector in contador1..vect1.len(){
                    order_list.push(vect1[vector].clone());
                }
            }
        }

        pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let mut order_list: Vec<Vec<i32>> = Vec::new();
            let mut contador1: usize = 0;
            let mut contador2: usize = 0;

            while contador1 < nums1.len() && contador2 < nums2.len() {
                let id1 = nums1[contador1][0];
                let id2 = nums2[contador2][0];
                let value1 = nums1[contador1][1];
                let value2 = nums2[contador2][1];

                if  id1 == id2 {
                    let sumados = vec![id1, value1 + value2];
                    order_list.push(sumados);
                    contador1 += 1;
                    contador2 += 1;

                } else if id1 < id2 {
                    order_list.push(vec![id1, value1]);
                    contador1 += 1;
                } else if id1 > id2 {
                    order_list.push(vec![id2, value2]);
                    contador2 += 1;
                }
            }
            Solution::return_all(&nums1, &nums2, contador1, contador2, &mut order_list);
            order_list
        }
    }
}