# Solucion del reto No.2523 de leetcode

# Importaciones
import math


class Solution:
    def __init__(self):
        self.limit = (10**6)+1
        self.list_prime = self._criba()

    def _criba(self) -> list:
        numbers_list = list(range(self.limit))
        for num in range(math.floor(math.sqrt(self.limit))):
            if num < 2:
                numbers_list[num] = False
                continue
            if not numbers_list[num]:
                continue
            for index_multiple in range(num*2, self.limit, num):
                numbers_list[index_multiple] = False
        return numbers_list

    def _get_prime(self, left: int, right: int):
        prime_in_range = []
        if left < 2:
            left = 2
        for num in self.list_prime[left:right+1]:
            if num:
                prime_in_range.append(num)
        return prime_in_range

    def _minor_pair(self, prime_list: list):
        pairs = list(zip(prime_list, prime_list[1:]))
        menor = [self.limit, [-1, -1]]
        for pair in pairs:
            diff = pair[1] - pair[0]
            if diff < menor[0]:
                menor[0] = diff
                menor[1] = pair

        return list(menor[1])

    def closestPrimes(self, left: int, right: int) -> list:
        prime_list = self._get_prime(left, right)
        if len(prime_list) < 2 or left == right:
            return [-1, -1]
        return self._minor_pair(prime_list)
