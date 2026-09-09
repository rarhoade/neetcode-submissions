class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        value_map = { }
        for i in range(0, len(nums)):
            n = target - nums[i]
            value_map[n] = i
        print(value_map)
        for i in range(0, len(nums)):
            j = value_map.get(nums[i])
            if j is not None and j != i:
                return [i, j]
        
        
        