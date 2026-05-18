class Solution:
    def maxProfit(self, prices: list[int]) -> int:
        lowest_value_and_index = [prices[0], 0]
        highest_value_and_index = [0, 0]
        diff = 0
        for idx, x in enumerate(prices):
            if x < lowest_value_and_index[0]:
                lowest_value_and_index[0] = x
                lowest_value_and_index[1] = idx
            if x - lowest_value_and_index[0] > diff and idx > 0:
                highest_value_and_index[0] = x
                highest_value_and_index[1] = idx
            if (
                highest_value_and_index[0] > lowest_value_and_index[0]
                and highest_value_and_index[1] > lowest_value_and_index[1]
            ):
                temp_diff = highest_value_and_index[0] - lowest_value_and_index[0]
                if temp_diff > diff:
                    diff = temp_diff
                continue

        return diff


# bug check that the following lowest is before a highest!
# check if diff highest lowest is biggest
if __name__ == "__main__":
    sol = Solution()
    prices = [3, 3, 5, 0, 0, 3, 1, 4]
    result = sol.maxProfit(prices)
    print(result)
