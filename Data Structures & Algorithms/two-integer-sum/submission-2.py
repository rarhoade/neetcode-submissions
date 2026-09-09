class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        value_map = { }
        for i in range(0, len(nums)):
            value = target - nums[i]
            value_map[value] = i
        for i in range(0, len(nums)):
            if value_map.get(nums[i]) is not None and value_map[nums[i]] is not i:
                return [i, value_map[nums[i]]]
        