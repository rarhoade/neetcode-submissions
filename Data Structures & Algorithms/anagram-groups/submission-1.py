class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        word_map = defaultdict(list)
        for word in strs:
            arr = [0] * 26
            for c in word:
                arr[ord(c) - ord("a")] += 1
            word_map[tuple(arr)].append(word)
        return word_map.values()
        
