# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        res = True
        #left = dfsHeight(root.left)
        #right = dfsHeight(root.right)
        # if abs(left - right) > 1:
        #   return False
        def dfsHeight(root):
            nonlocal res

            if not root:
                return 0
            left = dfsHeight(root.left)
            right = dfsHeight(root.right)
            if abs(left - right) > 1:
                res = False
            return 1 + max(left, right)
        dfsHeight(root)
        return res