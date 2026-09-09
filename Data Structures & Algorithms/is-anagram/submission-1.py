class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        s_map = {}
        t_map = {}

        for i in s:
            if s_map.get(i) is not None:
                s_map[i] = s_map[i] + 1
            else:
                s_map[i] = 1
        
        for i in t:
            if t_map.get(i) is not None:
                t_map[i] = t_map[i] + 1
            else:
                t_map[i] = 1
                
        if s_map == t_map:
            return True
        return False