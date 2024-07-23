fn main() {
    let matrix_a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    let matrix_b = vec![vec![9, 8, 7], vec![6, 5, 4], vec![3, 2, 1]];

    match matrix_multiplication(&matrix_a, &matrix_b) {
        Ok(result) => {
            println!("Result:");
            for row in result {
                println!("{:?}", row);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn matrix_multiplication(
    a: &Vec<Vec<i32>>,
    b: &Vec<Vec<i32>>,
) -> Result<Vec<Vec<i32>>, &'static str> {
    // Check if matrices can be multiplied
    if a[0].len() != b.len() {
        return Err("Matrices cannot be multiplied");
    }

    let rows_a = a.len();
    let cols_a = a[0].len();
    let cols_b = b[0].len();

    // Create the result matrix with the appropriate size
    let mut result = vec![vec![0; cols_b]; rows_a];

    // Perform the multiplication
    for i in 0..rows_a {
        for j in 0..cols_b {
            for k in 0..cols_a {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    Ok(result)
}
