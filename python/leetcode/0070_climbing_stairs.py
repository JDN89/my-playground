# https://www.youtube.com/watch?v=Y0lT9Fck7qI -> dynamic programming solution -> learned it
class Solution:
    def climbStairs(self, n: int) -> int:
        # 1 (1 way)
        #
        # 1
        #
        # 2 (2 ways)
        #
        # 2
        # 1 1
        #
        # 3 (3 ways 1 +2)
        #
        # 1 1 1
        # 1 2
        # 2 1
        #
        # 4 (5 ways 3 + 2) -> is a fibonacci sequence
        #
        # 1 1 1 1
        # 1 1 2
        # 2 1 1
        # 1 2 1
        # 2 2
        #

        # Recursion slow
        # base case
        # if n <= 2:
        #     return n
        # return self.climbStairs(n - 1) + self.climbStairs(n - 2)

        # fast neetcode solution
        one, two = 1, 1
        #
        # start fom the bottom up.
        # in how many ways can you reach the last step 1
        # in how many ways can you go from n-1 to n? 1
        # in how many ways can you go from n-2 to n? n-1 + n
        # and so for for (n-1) times
        #
        for i in range(n - 1):
            temp = one
            one = one + two
            two = temp
        return one
