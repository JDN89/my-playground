class Solution:
    def hammingWeight(self, n: int) -> int:
        count = 0
        while n:
            # n &1 controleert of bit even of oneven is
            # add to count when bit is odd
            count += n & 1
            # shift to the next bit in n from right to left
            n >>= 1
        return count
