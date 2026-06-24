use std::vec;

fn main() {
    struct Solution {
        nums1: Vec<Vec<i32>>,
        nums2: Vec<Vec<i32>>
    }

    impl Solution {
        pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let mut order_list: Vec<Vec<i32>> = Vec::new();
            let mut contador1: usize = 0;
            let mut contador2: usize = 0;

            loop {
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
                    if contador1 == (nums1.len() - 1){
                        order_list.push(vec![id2, value2]);
                        contador2 += 1;
                        continue;
                    }
                    order_list.push(vec![id1, value1]);
                    contador1 += 1;
                } else if id1 > id2 {
                    if contador2 == (nums2.len() - 1){
                        order_list.push(vec![id1, value1]);
                        contador1 += 1;
                        continue;
                    }
                    order_list.push(vec![id2, value2]);
                    contador2 += 1;
                }
                if contador2 == (nums2.len() - 1) && contador1 == (nums1.len() - 1) {
                    break order_list;
                }
            }
        }
    }
}