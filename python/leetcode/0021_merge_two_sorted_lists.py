# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

        # TODO find a good way to walk listnodes
        # Can't i just walk l1 and return l1 and modify l1? -> better to create new list I think
        # just walk first list, if you come at end of first list
        # and second list still has values -> append the rest to next
        # and vice versa
        #
        # init listnode append l1 or l2 as head and then start looping?


class Solution:
    def mergeTwoLists(
        self, list1: ListNode | None, list2: ListNode | None
    ) -> ListNode | None:

        if list1 is None:
            return list2
        if list2 is None:
            return list1

        if list1 is not None and list2 is not None:
            merged: ListNode = ListNode(0)  # merged is pointer to
            current = merged

            curr_l1 = list1
            curr_l2 = list2

            while curr_l1 and curr_l2:
                if curr_l1.val < curr_l2.val:
                    current.next = curr_l1
                    curr_l1 = curr_l1.next
                else:
                    current.next = curr_l2
                    curr_l2 = curr_l2.next
                current = current.next

            # append remaning nodes
            if curr_l1:
                current.next = curr_l1
            if curr_l2:
                current.next = curr_l2

        # return merged without the 0 initialized head
        return merged.next
