# Notas de solución

*Al ser mas de un archivo de solucion por lenguaje de programacion se encontraran en carpetas separadas.*

---

## solución python

**Fecha: 2026-04-28**

Intente la implementacion de la solucion haciendo la medicion individual de cada numero primo en el rango dado.

El problema principal es que cuando se solicite el calculo de muchos primos, el codigo exede el tiempo de respuesta. Lo ideal es un preprocesamiento de una lista de todos los primos posibles en el rango de:

right = 10^6.

La proxima implementacion sera la solucion con preprocesamiento de la criba de eratostenes para mayor velocidad (incluyendo su version en rust)