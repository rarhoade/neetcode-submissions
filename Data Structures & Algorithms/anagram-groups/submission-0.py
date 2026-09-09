class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        def generate_anagram_arr(string: str):
            arr = [0] * 26
            for i in string:
                arr[ord(i) - ord("a")] += 1
            return arr
        word_map = defaultdict(list)
        for word in strs:
            count_arr = generate_anagram_arr(word)
            word_map[tuple(count_arr)].append(word)
        return word_map.values()
        
