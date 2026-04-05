class Solution:
    def reverseBits(n: int) -> int:
        result = 0  # This will store the reversed bits

        for _ in range(32):  # Loop through all 32 bits
            # take last bit of n
            # | or ->
            # shift bits in n to the left -> append to the right
            result = (result << 1) | (n & 1)

            # move the the next bit in n, from right to left
            n >>= 1

        return result  # Return the reversed 32-bit integer
