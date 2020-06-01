enum Direction {
    Right,
    Down,
    Left,
    Up,
}

use Direction::*;

pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
    let (mut row, mut col) = (0, 0);
    let mut matrix = vec!(vec!(0_u32; size); size);
    let mut direction = Right;
    for counter in 1..=size * size {
        matrix[row][col] = counter as u32;
        match direction {
            Right => if col < size - 1 && matrix[row][col + 1] == 0{
                col += 1;
            } else {
                direction = Down;
                row += 1;
            },
            Down => if row < size - 1 && matrix[row + 1][col] == 0{
                row +=1;
            } else {
                direction = Left;
                col -=1;
            },
            Left => if col > 0 && matrix[row][col - 1] == 0{
                col -=1;
            } else {
                direction = Up;
                row -= 1;
            },
            Up => if row > 0 && matrix[row - 1][col] == 0{
                row -=1;
            } else {
                direction = Right;
                col += 1;
            }
        }
    }
    matrix
}
