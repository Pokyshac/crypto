use std::ops::DerefMut;

pub fn get_multiplication_table(m: usize) -> Vec<usize> {
    let mut table = vec![0; m * m];
    for i in 0..m {
        for j in i..m {
            table[i * m + j] = (i * j) % m;
            table[j * m + i] = (i * j) % m;
        }
    }

    table
}

pub fn get_adding_table(m: usize) -> Vec<usize> {
    let mut table = vec![0; m * m];
    for i in 0..m {
        for j in i..m {
            table[i * m + j] = (i + j) % m;
            table[j * m + i] = (i + j) % m;
        }
    }

    table
}
