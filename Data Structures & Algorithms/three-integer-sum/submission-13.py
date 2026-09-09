class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        nums.sort()
        res = []
        for i in range(0, len(nums) - 2):
            if nums[i] > 0:
                break
            
            if i > 0 and nums[i] == nums[i - 1]:
                continue 
                
            j = i + 1
            k = len(nums) - 1
            target = -nums[i]
            while j < k:
                summation = nums[j] + nums[k]
                if target > summation:
                    j = j + 1
                elif target < summation:
                    k = k - 1
                else:
                    res.append([nums[i], nums[j], nums[k]])
                    j = j + 1
                    k = k - 1
                    while nums[j] == nums[j - 1] and j < k:
                        j = j + 1
        return res
        