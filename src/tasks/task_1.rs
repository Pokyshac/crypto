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

pub fn table_to_string(table: &Vec<usize>, size: usize) -> Result<String, &str> {
    if size == 0 {
        return Err("Size can not be zero");
    }
    
    let cell_size = digits_count(
            table.iter()
                .max()
                .unwrap()
                .clone()
        ) + 1;
    
    let mut result = String::with_capacity((cell_size + 1) * size * 2);
    let mut k = 0;
    let mut row = String::with_capacity((cell_size + 1) * size);
    for i in 0..size * 2 {
        row.clear();
        if i % 2 == 0 {
            for j in 0..size {
                let string_value = table[k].to_string();
                let spaces_count = cell_size - string_value.len();
                let spaces = vec![" "; spaces_count].join("");
                let cell = string_value + &spaces + "|";
                
                row.push_str(&cell);
                
                k += 1;
            }
        }
        else {
            for j in 0..size {
                let cell = vec!["-"; cell_size].join("") + "|";
                
                row.push_str(&cell);
            }
        }
        result.push_str(&row);
        result.push_str("\n");
    }
    
    Ok(result)
}

fn digits_count(value: usize) -> usize {
    let mut digits_count = 0;
    let mut value = value;
    while value > 0 {
        digits_count += 1;
        value /= 10;
    }

    digits_count
}
