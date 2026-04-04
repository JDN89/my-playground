class Solution:
    def missingNumber(self, nums: list[int]) -> int:
        n = len(nums)
        total = n * (n + 1) / 2
        numsSum = sum(nums)
        missingNumber = total - numsSum
        return int(missingNumber)
