class Solution:
    def twoSum(self, nums: list[int], target: int) -> list[int]:
        MEMORIA = {}
        for index, numero in enumerate(nums):
            busca = target - numero
            if busca in MEMORIA:
                return [MEMORIA[busca], index]
            MEMORIA[numero] = index


# Prueba la cual tiene que dar [3,4] para ser correcta
prueba = Solution().twoSum([1, 3, 54, 6, 2, 1, 3, 6, 7], 8)
print(prueba)  # impresion de la prueba
