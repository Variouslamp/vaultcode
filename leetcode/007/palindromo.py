class Solution:
    def isPalindrome(self, x: int) -> bool:
        if not x < 0:
            n = str(x)
            temp = []
            for digito in n:
                temp.insert(0, digito)
            x2 = "".join(temp)
            if int(x2) == int(x):
                return True
            return False
        return False


num = input(": ")
pal = Solution()
print(pal.isPalindrome(num))
