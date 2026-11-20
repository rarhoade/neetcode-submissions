impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
        let mut checks: HashSet<char> = HashSet::new();
        for j in 0..9 {
            if board[i][j] == '.' {
                continue;
            }
            if !checks.insert(board[i][j]) {
                return false;
            }
        }
    }

    for i in 0..9 {
        let mut checks: HashSet<char> = HashSet::new();
        for j in 0..9 {
            if board[j][i] == '.' {
                continue;
            }
            if !checks.insert(board[j][i]) {
                return false;
            }
        }
    }

    for square in 0..9 {
        let mut checks = HashSet::new();
        for i in 0..3 {
            for j in 0..3 {
                let row = (square / 3) * 3 + i;
                let col = (square % 3) * 3 + j;
                if board[row][col] == '.' {
                    continue;
                }
                if !checks.insert(board[row][col]) {
                    return false;
                }
            }
        }
    }
    return true;
    }
}
