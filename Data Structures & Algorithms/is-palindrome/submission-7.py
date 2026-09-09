class Solution:
    def isPalindrome(self, s: str) -> bool:
        i = 0
        j = len(s) - 1
        while i < j:
            if (not s[i].isalnum()) or s[i].isspace():
                print("I", s[i])
                i += 1
            elif (not s[j].isalnum()) or s[j].isspace():
                print("J", s[j])
                j -= 1
            else:
                print(s[i], s[j])
                if s[i].lower() != s[j].lower():
                    return False
                i += 1
                j -= 1
        return True