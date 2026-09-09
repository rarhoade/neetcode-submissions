class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        blank_map = { }
        for i in range(1, 10):
            blank_map[str(i)] = False
        # rows
        for i in range(len(board)):
            curr_map = blank_map.copy()
            curr_row = board[i]
            for j in range(0, len(curr_row)):
                if curr_row[j] != '.':
                    if curr_map[curr_row[j]] is False:
                        curr_map[curr_row[j]] = True
                    else:
                        return False
        for i in range(0, len(board)):
            curr_map = blank_map.copy()
            for j in range(0, len(board)):
                val = board[j][i]
                if board[j][i] != '.':
                    if curr_map[val] != '.':
                        if curr_map[val] is False:
                            curr_map[val] = True
                        else:
                            return False
        x = 0
        while x < len(board):
            curr_map = blank_map.copy()
            y = x
            while y < x + 3:
                z = x
                while z < x + 3:
                    val = board[y][z]
                    if val != '.':
                        if curr_map[val] != '.':
                            if curr_map[val] is False:
                                curr_map[val] = True
                            else:
                                return False
                    z += 1
                y += 1
            x += 3

        return True
                

        