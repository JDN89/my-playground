class Solution:
    def twoSum(self, nums: list[int], target: int) -> list[int]:
        """
        create dictionary
        enumerate over the list
        subtract target - current val
        if allready in dictionary, return the indexes
        otherwise add the current val and it's index
        subtract target - current val in enumeration
        if allready in map return
        otherwise add
        """

        seen = {}
        for i, num in enumerate(nums):
            diff = target - num  #
            if diff in seen:
                return [seen[diff], i]
            seen[num] = i

        return []


def main():
    sol = Solution()
    nums = [2, 7, 11, 15]
    target = 9
    print(sol.twoSum(nums, target))


if __name__ == "__main__":
    main()
