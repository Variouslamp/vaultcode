# Notas de solución

---

## solución 

**Fecha: 2026-06-30**

Durante la crecion de este problema me vi bastante acorralado la verdad ya que yo no tenia experiencia tratando con el manejo de listas enlazadas de una manera como lo solicita este problema, que es tomar los datos de las listas voltearlos y sumarlo y retornar un valor en el mismo formato que se nos entrego, mi estructura mental al leer el problema y lo que requeria fue la siguiente:

1- obtener los numeros de cada una de las listas enlazadas
2- rotarlos para que queden en el valor real y no el "espejo"
3- Juntar los numeros y convertirlos en uno que pueda sumarte a el que se dio en la otra lista
4- con el resultado de la suma separamos los numeros
5- giramos los numeros como "espejo" de nuevo
6- creamos la lista enlazada con la estructura ya definida

La creacion fue interesante me obligo a usar varios metodos para recorrer la lista tranformas los datos y reconstruirlos a su forma inicial.

He leido y en las propias soluciones de leetcode se muestran maneras mas eficientes haciendo uso de una tecnica llamada "carry" que es una suma de los nodos en si e ir creandolos mientras se hacen las operaciones, la solucion en RUST sera hecha de esa manera mas eficiente, puede que tambien la haga en python.

**Fecha: 2026-07-02**

Haciendo una investigacion ya he analizado que de los puntos fuertes de la solucion optimas es hacer las suma de los nodos de manera continua sin hacer ningun tipo de conversion de tipo como a String o sinsiquiera teniendo que juntar los valores de los nodos haciendo uso de un "carry" que funciona como guardar los numeros grandes de una suma unicamete colocando la unidad para dejar decena sumada para el siguiente numero.

Voy a implementar esta soluycion para poder entender la logica de manera mas profunda.