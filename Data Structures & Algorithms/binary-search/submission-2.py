class Solution:
    def search(self, nums: List[int], target: int) -> int:
        left = 0
        right = len(nums) - 1
        while left <= right:
            middle = left + ((right - left) // 2)
            curr = nums[middle]
            print(curr, middle)
            if curr == target:
                return middle
            elif curr < target:
                # go to the right
                left = middle + 1
            else:
                right = middle - 1
        return -1