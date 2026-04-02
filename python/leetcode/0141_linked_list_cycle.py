# Definition for singly-linked list.


class ListNode:
    def __init__(self, x: int = 0):
        self.val = x
        self.next = None


def build_list(values: list[int]):
    dummy = ListNode(0)  # or ListNode() if you added default
    cur = dummy

    for v in values:
        if cur.next is None:
            cur.next = ListNode(v)  # create new node
        cur = cur.next  # move pointer forward

    return dummy.next


# python objects implement hash and eq
# store the object in set
# set is achterliggen hastable
class Solution:
    def hasCycle(self, head: ListNode | None) -> bool:
        if head is None:
            return False

        seen = set()
        curr: ListNode = head
        while curr:
            if curr in seen:
                return True
            seen.add(curr)
            curr = curr.next
        return False


if __name__ == "__main__":
    sol = Solution()
    head = build_list([3, 2, 0, -4])
    print(sol.hasCycle(head))
