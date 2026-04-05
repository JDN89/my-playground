class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


# compare function
# keep looping until we reach the end.
# IF we reach the end, that means all nodes where the same
def compare(root: TreeNode | None, subRoot: TreeNode | None) -> bool:
    # this is the basecase -> loop stops when we reach the last nodes (and haven't encountered false)
    if not root and not subRoot:
        print("we reached the end")
        return True
    if not root or not subRoot:
        print("Root or subroot is empty")
        return False
    if root.val != subRoot.val:
        print("Root.val or subroot.val are not the same")
        return False
    # recurse until we reach one of the end cases
    return compare(root.left, subRoot.left) and compare(root.right, subRoot.right)


# we keep calling compare until we reach the end
# if they don't match before end -> we shortcircuit and return false
# call isSubTree again
#
# if no root and subroot -> True
# if no root or subroot -> False
# if not same value -> false


# DFS explore left branch, when no left -> go right
def walkTree(t: TreeNode):
    print(t.val)
    if t.left:
        walkTree(t.left)
    if t.right:
        walkTree(t.right)


class Solution:
    def isSubtree(self, root: TreeNode | None, subRoot: TreeNode | None) -> bool:
        if not root:
            return False
        if compare(root, subRoot):
            return True
            # if compare root and subroot
            # return true
        return self.isSubtree(root.left, subRoot) or self.isSubtree(root.right, subRoot)

    # return isSubtree root left, sub or root right, sub
