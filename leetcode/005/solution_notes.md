# Notas de solución

---

## solución 

**Fecha: 2026-06-29**

Durante el planteamiento de la solucion se intenta mantener una solucion super eficiente, lo cual me llevo al planteamiento de una solucion que no conlleve blucles anidados, la idea mas sencilla y principal es hacer simplemente una comparacion entre el numero que complementa a otro, por ejemplo el codigo que habia planteado yo en un inicio y el mas sencillo pero a su vez el menos eficiente es el siguiente:

```python
for index1, i in enumerate(lista_valores):
    for index2, j in enumerate(lista_valores):
        if index2 == index1:
            continue
        if i + j == valor_buscado:
            return [index1, index2]
```
El cual es muy funcional ya que su logica hace exactamente lo que nececitamos, por cada uno de los valores dentro del array, lo suma con cada uno de los valores y cuando alguna de dichas sumas de como resultado el valor que estamos buscando nos retorna los indice de ambos.

pero hay un problema de eficiencia ya que este algoritmo tiene una complegidad algoritmica de 

 O(n**2) 

 haciendo que para muy grandes volumenes de datos sea altamente ineficiente, asi que decidi elegir otro tipo de algoritmo, uno que memorizara cuales de los elementos anteriores ya fueron visitados, entonces lo que hace es que cuando visita un valor nuevo y hace la resta al valor que estamos buscando ya sabemos por que numero es el que tenemos que sumarlo entonces se busca en el diccionario definido al inicio a ver si esta dicho valor y si no esta, el valor actual es almacenado en el diccionario continuando con el siguiente y volviendo a ejecutar el bucle, en el momento que uno de los valores pregunte por otro valor que ya haya sido visitado antes este devolvera el indice de ambos. 

