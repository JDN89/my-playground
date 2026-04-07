# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution:
    def reverseList(self, head: ListNode | None) -> ListNode | None:

        prev, curr = None, head

        while curr:
            # de curr node pointen naar de vorige node.
            # De next node slaan we tijdelijk op, omdat eens de link inverted is we niet naar de next node kunnen gaan via.next
            # store the next node
            next = curr.next
            curr.next = prev  # point to the previous node to break the link
            prev = curr  # we moven prev op naar curr node
            curr = next  # curr node verschuiven naar de volgende en dan alles terug opnieuw doen
        return prev
