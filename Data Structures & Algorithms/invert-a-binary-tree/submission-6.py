# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        ### dfs solution

        # base case
        if not root:
            return None
        
        # create queue FIFO
        queue = deque([root])

        while queue:
            #take from the left
            node = queue.popleft()

            #swap nodes
            node.left, node.right = node.right, node.left
            
            #add to queue
            if node.left:
                queue.append(node.left)
            if node.right:
                queue.append(node.right)
        return root