fn main() {
    struct Solution {
        nums: Vec<i32>
    }

    impl Solution {
        pub fn pivot_index(nums: Vec<i32>) -> i32 {
            let longitud_vector = nums.len();
            let suma_vector: i32 = nums.iter().sum();
            let mitad_suma = (suma_vector / 2) as f64;
            let mitad_suma = mitad_suma.round();
            let mut izquierda: usize = 0;
            let mut derecha: usize = longitud_vector - 1;
            let mut indice_central: usize;
            let mut pivote: i32;
            let mut indice_anterior: usize = 0;

            loop{
                indice_central = (izquierda + derecha) / 2;
                pivote = indice_central as i32;
                let valor_indice_central = nums[indice_central] as f64;
                let lado_esperado = mitad_suma - (valor_indice_central/ 2.0 );
                let lado_suma: i32 = nums[..indice_central].iter().sum();
                let lado_suma = lado_suma as f64;

                if lado_suma == lado_esperado {
                    break;
                } else if lado_suma < lado_esperado {
                    izquierda = indice_central + 1;
                } else if lado_suma > lado_esperado {
                    derecha = indice_central - 1;
                }
                if indice_anterior == indice_central {
                    pivote = -1;
                    break;
                }
                indice_anterior = indice_central
            }

            
            pivote
        }
    }

    // pruebas de la funcion
    let instancia = Solution::pivot_index(vec![1,5,-2,7,2]);
    print!("pivote = {}",instancia)
}