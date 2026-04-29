# Notas de solución

*Al ser mas de un archivo de solucion por lenguaje de programacion se encontraran en carpetas separadas.*

---

## solución python

**Fecha: 2026-04-28**

Intente la implementacion de la solucion haciendo la medicion individual de cada numero primo en el rango dado.

El problema principal es que cuando se solicite el calculo de muchos primos, el codigo exede el tiempo de respuesta. Lo ideal es un preprocesamiento de una lista de todos los primos posibles en el rango de:

right = 10^6.

La proxima implementacion sera la solucion con preprocesamiento de la criba de eratostenes para mayor velocidad (incluyendo su version en rust)

**Fecha: 2026-04-29**

implemente la solucion enfocada en la criba de eratostener para sacar la lista de valores primos y enfocarse en la seleccion de los valores con base en su indice, la verdad es interesante la creacion de una misma solucion con diferentes metodos, pero el tema es que no note una gran diferencia en la eficiencia del algoritmo respecto a ms para encontrar una misma pareja de primos haciendo uso de cualquiera de los dos algoritmos.

