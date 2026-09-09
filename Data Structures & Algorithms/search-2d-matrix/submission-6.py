class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        m = 0
        arr = matrix[0]
        low, high = 0, len(arr) - 1
        while m < len(matrix):
            arr = matrix[m]
            if arr[low] <= target and arr[high] >= target:
                break
            else:
                m += 1
        print(arr)
        while low <= high:
            mid = (low + high) // 2
            if arr[mid] == target:
                return True
            elif arr[mid] < target:
                low = mid + 1
            else:
                high = mid - 1
        return False
        