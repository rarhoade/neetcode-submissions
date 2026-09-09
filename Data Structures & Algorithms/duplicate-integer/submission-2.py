class Solution:
    def hasDuplicate(self, nums: List[int]) -> bool:
        store_hash = {}
        for val in nums:
            if val in store_hash:
                return True
            store_hash[val] = True
        return False