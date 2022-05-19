
namespace leetcode_csharp.Problems.MergeSortedList;


/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     public int val;
 *     public ListNode next;
 *     public ListNode(int val=0, ListNode next=null) {
 *         this.val = val;
 *         this.next = next;
 *     }
 * }
 */

/*
 you get the heads of two linked lists
 given the HEADS of two sorted linked lists, create a combined sorted linked list, and return the HEAD of the combined sorted linked list?
 l1 or l2 or the heads of two linked lists
 you have to return the head of the merged lists
 with recursion, you keep calling the merge function until you reach null at wich point your return l1 or l2
 in recursion you end with the base case
 
    public ListNode MergeTwoLists(ListNode list1, ListNode list2) {
        
        if (list1 is null) return list2;
        if (list2 is null) return list1;
        
        if (list1.val<= list2.val) {
        
            list1.next = MergeTwoLists(list1.next, list2);
            return list1;
        }
        else {
            list2.next= MergeTwoLists(list2.next,list1);
            return list2;
        }

*/
