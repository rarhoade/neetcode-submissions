class Solution:
    def hasDuplicate(self, nums: List[int]) -> bool:
        nums_check = {}
        for n in nums:
            if nums_check.get(n) is None:
                nums_check[n] = True
            else:
                return True
        return False