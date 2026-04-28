# Solucion del reto No.2523 de leetcode

# Importaciones
import math

# -----------------------------------------------------------------------------
# Funcion que verifica si un numero es o no primo


def primo(numero: int) -> bool:
    if numero == 2:
        return True
    if numero <= 2 or numero % 2 == 0:
        return False
    else:
        for num in range(3, math.floor(math.sqrt(numero))+1, 2):
            if numero % num == 0:
                return False
        return True

# -----------------------------------------------------------------------------
# Funcion que verifica el par de numero primos que sumados dan el menor numero
# en un intervalo de dos valores dados (left, right)


def retorno_lista(left: int, right: int):
    resultado = []
    for i in range(left, right+1):
        if primo(i):
            resultado.append(i)
    if len(resultado) < 2:
        return [-1, -1]
    return resultado

# -----------------------------------------------------------------------------
# Funcion que retorna la pareja de numeros primos con la diferencia entre ellos
# mas pequeña


def pareja_menor(lista: list):
    pares = []
    pareja = []
    for i in lista:
        pareja.append(i)
        if len(pareja) == 2:
            pares.append(pareja)
            pareja = [i]
    diff = []
    for par in pares:
        diff.append(par[1] - par[0])
    minimo = min(diff)
    indice = diff.index(minimo)
    return pares[indice]

# -----------------------------------------------------------------------------
# Funcion main que junta las funciones


def main():
    left = int(input('left: '))
    right = int(input('right: '))
    lista = retorno_lista(left, right)
    if lista != [-1, -1]:
        lista = pareja_menor(lista)
    return print(lista)

# -----------------------------------------------------------------------------


main()  # Llamada de main que corre todo el codigo
