class Solution:
    def countBits(self, n: int) -> list[int]:
        bitsArray = []
        for i in range(n + 1):
            occurences = bin(i)[2:].count("1")
            bitsArray.append(occurences)
        print(bitsArray)

        return bitsArray


# o(n) solution via chatgpt
def countBits(n: int):
    dp = [0] * (n + 1)

    for i in range(1, n + 1):
        # >> is division by two floor
        # i & 1 get last bit (0 or 1)
        dp[i] = dp[i >> 1] + (i & 1)

    return dp


# 0 → 0
# 1 → 1
# 2 → 1   >> 1 0 so 2 divided by 2 [1] -> 1 has 1 bit and last bit of 2 is 0 -> 1
# 3 → 2   >> 11  so 3 /2 is 1 (floor) -> [1] is 1 and last bit of 3 is 1 : 1+ 1 = 2
# 4 → 1   >> and so forth
# 5 → 2
