# use set
# no duplicates
# instant lookup
# loop over and put in set
class Solution:
    def longestConsecutive(self, nums: list[int]) -> int:
        if len(nums) == 0:
            return 0
        num_set = set()
        for num in nums:
            num_set.add(num)
        start_sequences: list[int] = [x for x in num_set if x - 1 not in num_set]
        counter = 1
        temp_counter = 1

        if len(start_sequences) > 0:
            for x in start_sequences:
                temp_counter = 1
                while x + 1 in num_set:
                    temp_counter += 1
                    x += 1
                if temp_counter > counter:
                    counter = temp_counter

        return counter


if __name__ == "__main__":
    sol = Solution()
    input: list[int] = [0, 3, 7, 2, 5, 8, 4, 6, 0, 1]
    result = sol.longestConsecutive(input)
    print(result)
