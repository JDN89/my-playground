class Solution:
    def countBits(self, n: int) -> list[int]:
        bitsArray = []
        for i in range(n + 1):
            occurences = bin(i)[2:].count("1")
            bitsArray.append(occurences)
        print(bitsArray)

        return bitsArray
