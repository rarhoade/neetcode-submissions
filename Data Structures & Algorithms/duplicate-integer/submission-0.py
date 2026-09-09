class Solution:
    def hasDuplicate(self, nums: List[int]) -> bool:
        dup_map = {}
        for n in nums:
            if dup_map.get(n) is None:
                dup_map[n] = True
            else:
                return True
        return False