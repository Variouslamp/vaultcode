# Solucion del reto No.2523 de leetcode
# Este archivo tiene una estructura diferente ya que es la estructura que
# permite ingresar dentro de la plataforma de leetcode

# Importaciones
from typing import List
import math


class Solution:

    # -------------------------------------------------------------------------
    # Funcion que verifica si un numero es o no primo

    def primo(self, numero: int) -> bool:
        if numero == 2:
            return True
        if numero <= 2 or numero % 2 == 0:
            return False
        else:
            for num in range(3, math.floor(math.sqrt(numero)) + 1):
                if numero % num == 0:
                    return False
            return True

    # -------------------------------------------------------------------------
    # Funcion que verifica el par de numero primos que sumados dan el menor
    # numero en un intervalo de dos valores dados (left, right)

    def retorno_lista(self, left: int, right: int):
        resultado = []
        for i in range(left, right + 1):
            if self.primo(i):
                resultado.append(i)
        if len(resultado) < 2:
            return [-1, -1]
        return resultado

    # -------------------------------------------------------------------------
    # Funcion que retorna la pareja de numeros primos con la diferencia entre
    # ellos mas pequeña

    def pareja_menor(self, lista: list):
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

    # -------------------------------------------------------------------------
    # Funcion principal requerida por LeetCode

    def closestPrimes(self, left: int, right: int) -> List[int]:
        lista = self.retorno_lista(left, right)

        if lista != [-1, -1]:
            lista = self.pareja_menor(lista)

        return lista
