class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        s_store = {}
        for i in s:
            if i in s_store:
                s_store[i] = s_store[i] + 1
            else:
                s_store[i] = 1
        
        for i in t:
            if i in s_store:
                s_store[i] = s_store[i] - 1
                if s_store[i] < 0:
                    return False
            else:
                return False
        for key, val in s_store.items():
            if s_store[key] != 0:
                return False
        
        return True