# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def swap(root: TreeNode):
    if root.left and root.right:
        print("root left and right present")
        print(root.left)
        print(root.right)

        temp_left = root.left
        temp_right = root.right
        root.left = temp_right
        swap(root.left)
        root.right = temp_left
        swap(root.right)
    elif root.left:
        root.right = root.left
        root.left = None
        swap(root.right)
    elif root.right:
        root.left = root.right
        root.right = None
        swap(root.left)
    return


class Solution:
    # start at leaves and invert?
    def invertTree(self, root: TreeNode | None) -> TreeNode | None:
        if root:
            swap(root)
            return root
