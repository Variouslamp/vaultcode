# Notas de solución

---

## solución 

**Fecha: 2026-07-02**
### Ideas de implementacion primera leida de reto

Esta solucion es la primera que se me ocurre al leer el problema, asi que sera testeada en problema dependiendo de su desempeño. A continuacion se colocara el flujo que se planea seguir en el algoritmo

- Definir un contador que comience en 1
- Define variable "longest_substring" inicializada en 0
- Iterar el string letra por letra
- Cada iteracion el contador suma 1
- Cada iteracion se almacenara la letra en un hashmap
- SI UNA LETRA SE REPITE
- Se compara con el contenido de "longest_substring" y si es mayor se ingresa en la variable 
- Se reestablece el contador en 1
- Se limpia el hashmap
- Continua la iteracion
- Al momento de que se finalice la iteracion se tetorna el contenido de "longest_substring"

Segun el primer vistazo que le hice al problema pienso que es una solucion bastante solida ya que cumple con los requisitos de devolver el valor del substring mas largo presente en el texto unicamente iterando una unica vez todo el string lo cual lo hace muy eficiente. segun mis conocimientos esta seria la tabla de The big O notation

- Iteracion del string O(n)
- Revision del hashmap O(1)
---

Se presentaron unos problemas con la implementacion anterior, funcionaba de una manera muy sencilla y eficiente pero solo bajo ciertas caracteristicas, el problema es que cuando por ejemplo se le ingresaba "dvdf" el cumple la siguiente logica

- revisa si 'd' esta, la almacena, suma al contador
- revisa si 'v' esta, la almacena, suma al contador
- resiva si 'd' esta, ESTA, almacena el contador(2), borra hash
- almacena 'd', suma al contador
- revisa si 'f' esta, lo almacena, sumal al contador
- ACABO LAS LETRAS 
- contador = 2 

El error es que cuando vuelve a comenzar despues de repetir la 'd' unicamente realmacena la d pero no revisa por ejemplo que cuando se repite la 'd' el invalida todo lo que se encuentra todo lo que esta delante de la primera 'd' es decir la 'v' letra la cual no se ha repetido aun, por lo que este diseño es poco eficiente.

Cuando vi este problema se me ocurrio otra idea a partid de la anterior conclucion, lo que queda invalido no es todo lo que esta detras de la segunda repeticion de una letra, es lo que esta detras de la primera, es decir se tiene que contar de nuevo un substring despues de la primera repeticion de una letra, asi que plantee lo siguiente,

¿Que mantenemos?
- se mantiene el almacenamiento del substring mas largo

¿Que cambiamos?
- Ya no se hara uso de un conteo reltaivo respecto a cuando se detecta el inicio de un substring sino sera un conteo general que funcione como un enumerate pero no se hace uso de este ya que nececito que empiece desde 1 no en 0.
- se hara un uso de doble puntero en vez de uno unico, estos dos punteros uno sera uno que apunte segun la iteracion y otro es mas bien teorico ya que simplemente es una variable la cual almacena unos datos que se encuentran dentro de la hashtable
- la hashtable no sera eliminada despues de hallar cada una de los substrings,sino que unicamente eliminara repeticiones de datos especificos que sean necesarios.
