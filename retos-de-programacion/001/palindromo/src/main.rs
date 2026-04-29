// ----------------------------------------------------------------------------
// Importaciones

use std::io; // Importe para poder ingreesar texto

// ----------------------------------------------------------------------------
// Funcion que se encarga de la reversion y retorno del texto "reverso"


fn voltear_texto(text: &str) -> String{ 
    let mut lista: Vec<char> = vec![];
    for i in text.chars() {
        lista.insert(0, i)
    }
    lista.iter().collect()
}


// ----------------------------------------------------------------------------
// Funcion que junta la logica y permte analizar si es o no palindromo


fn input() -> String{
    let mut input = String::new();  // Variable que almacenainput del usuario

    io::stdin()
        .read_line(&mut input)
        .expect("Error: problema al leer la linea");  // Agregar input usuario

    let input = input.trim(); // Se retira el salto de linea al final
    let input = input.replace(" ", "");  // Se retiran todos los espacios

    input
}


// ----------------------------------------------------------------------------
// Funcion que junta la logica y permte analizar si es o no palindromo


fn main(){
    println!(" ");
    println!("Ingrese Texto que crea que es un palindromo: ");
    let input = input();
    println!(" ");
    if input == voltear_texto(&input) {
        println!("Si, el texto es un palindromo")
    }else{println!("No, el texto no es un palindromo", )}
}