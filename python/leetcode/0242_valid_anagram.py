# check length
# shortest should be anagram of longest
# discard until match
# when you find first match
class Solution:
    def isAnagram(self, s: str, t: str) -> bool:

        s_len = len(s)
        t_len = len(t)
        # return false when s and t not same len or one of them is empty
        if s_len != t_len or s_len == 0:
            return False

        visited = dict()
        for ch in s:
            if ch in visited:
                visited[ch] += 1
            else:
                visited.setdefault(ch, 1)
        for ch in t:
            if ch not in visited or visited[ch] == 0:
                return False
            visited[ch] -= 1

        # # if of one values not == 0 -> return false
        # for val in visited.values():
        #     if val != 0:
        #         return False

        return True


if __name__ == "__main__":
    sol = Solution()
    result = sol.isAnagram("anagram", "nagaram")
    print(result)
