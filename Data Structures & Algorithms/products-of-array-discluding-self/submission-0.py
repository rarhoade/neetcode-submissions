class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        res = [ ]
        for i in range(0, len(nums)):
            value = 1
            j = i + 1
            while j < len(nums):
                value *= nums[j]
                j += 1
            j = i - 1
            while j >= 0:
                value *= nums[j]
                j -= 1
            res.append(value)
        return res