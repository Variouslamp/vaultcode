fn swap(
    fila1: usize, celda1: usize,
    fila2: usize, celda2: usize,
    matriz: &mut Vec<Vec<i32>>
) {
    let temp = matriz[fila1][celda1];
    matriz[fila1][celda1] = matriz[fila2][celda2];
    matriz[fila2][celda2] = temp;
}

fn main() {
    let mut matriz = vec![
        vec![1,2,3,4],
        vec![5,6,7,8],
        vec![9,10,11,12],
        vec![13,14,15,16]
    ];

    let n = matriz.len();
    let old_matriz = matriz.clone();

    for fila in 0..n{
        for celda in fila..n{
            swap(fila, celda, celda, fila, &mut matriz);
        }
    }
    for fila in 0..n{
        for celda in 0..n/2{
            let celda_nueva = n - 1 - celda;
            print!("{}",celda_nueva);
            swap(fila, celda, fila, celda_nueva, &mut matriz);
        }
    }
    for i in old_matriz{
        print!("{:?}\n", i)
    }
    print!("-----90 GRADOS A LA DERECHA-----\n"); 
    for i in matriz{
        print!("{:?}\n", i)
    }
}
