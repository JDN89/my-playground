class Solution:
    def isValid(self, s: str) -> bool:
        stack = []
        if len(s) <= 1 or s[0] in ")}]":
            print("to short")
            return False

        for c in s:
            if c in "([{":
                stack.append(c)
            elif len(stack) == 0:
                return False
            elif c == ")" and len(stack) > 0:
                if stack[-1] == "(":
                    print("popped (")
                    stack.pop()
                    continue
                else:
                    return False
            elif c == "]" and len(stack) > 0:
                if stack[-1] == "[":
                    print("popped [")
                    stack.pop()
                    continue
                else:
                    return False
            elif c == "}" and len(stack) > 0:
                if stack[-1] == "{":
                    print("popped {")
                    stack.pop()
                    continue
                else:
                    return False
        return len(stack) == 0
