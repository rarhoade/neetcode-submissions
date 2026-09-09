class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        nums.sort()
        triplets = []
        for x, a in enumerate(nums):
            if x > 0 and a == nums[x - 1]:
                continue
            i = x + 1
            j = len(nums) - 1
            while i < j:
                val = nums[x] + nums[i] + nums[j]
                if val > 0:
                    j -= 1
                elif val < 0:
                    i += 1
                else:
                    triplets.append([a, nums[i], nums[j]])
                    i += 1
                    while nums[i] == nums[i - 1] and i < j:
                        i += 1
        return triplets

