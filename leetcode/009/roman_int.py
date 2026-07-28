class Solution(object):
    def romanToInt(self, s):
        intiger=0

        VALUES = {
            "I": 1,
            "V": 5,
            "X": 10,
            "L": 50,
            "C": 100,
            "D": 500,
            "M": 1000
        }
        REPLACE = {
            "IV": 4,
            "IX": 9,
            "XL": 40,
            "XC": 90,
            "CD": 400,
            "CM": 900
        }
        for sustract in REPLACE:
            print(sustract)
            if sustract in s:
                s=s.replace(sustract, "")
                intiger += REPLACE[sustract]
        for symbol in s:
            if symbol in VALUES:
                intiger += VALUES[symbol]
        return intiger

a=Solution()

print(a.romanToInt("IIIIkkkkkkkk345I"))
