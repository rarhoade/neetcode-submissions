class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        frequency_count = {}
        for n in nums:
            if frequency_count.get(n) is None:
                frequency_count[n] = 1
            else:
                frequency_count[n] = frequency_count[n] + 1
        print(frequency_count)
        sorted_keys = sorted(frequency_count, key=frequency_count.get)
        return sorted_keys[-k:]