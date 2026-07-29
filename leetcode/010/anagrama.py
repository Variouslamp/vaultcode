class Solution(object):
    def isAnagram(self, s, t):
        if len(s) != len(t):
            return False
        for indice, letra in enumerate(s):
            t=t.replace( letra, "", 1 )
            s=s.replace( letra, "", 1 )
        if len(s) == len(t):
            return True
        else:
            return False

a=Solution()
a.isAnagram("khkokklkka", "alokkkkkkh")
