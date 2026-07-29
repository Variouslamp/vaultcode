class Solution(object):
    def isAnagram(self, s, t):
        if len(s) != len(t):
            return False
            
        count = {}
        
        for char_s, char_t in zip(s, t):
            count[char_s] = count.get(char_s, 0) + 1
            count[char_t] = count.get(char_t, 0) - 1
            
        for val in count.values():
            if val != 0:
                return False
                
        return True
