use core::iter::Iterator;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let rows = minefield.len();
    for (i, row) in minefield.iter().enumerate() {
        let cols = row.len();
        let mut output_row = String::new();
        for (j, &cell) in row.as_bytes().iter().enumerate() {
            let mut output_cell = ' ';
            if cell == b' ' {
                let mut neighbours: u32 = 0;
                let nimin = i.saturating_sub(1);
                let nimax = i.saturating_add(1);
                let njmin = j.saturating_sub(1);
                let njmax = j.saturating_add(1);
                for ni in nimin..=nimax {
                    for nj in njmin..=njmax {
                        println!("{rows}, {cols}");
                        if  ni < rows && nj < cols
                            && (ni, nj) != (i, j)
                            && minefield[ni].as_bytes()[nj] == b'*' {
                            neighbours += 1
                        }
                    }
                }
                if neighbours != 0 {
                    output_cell =
                        char::from_digit(neighbours, 10)
                        .expect("Maximum of 9 neighbours expected");
                }
            } else {
                output_cell =
                    char::from_u32(cell.into())
                    .expect("Only ASCII chars expected");
            }
            output_row.push(output_cell);
        }
        result.push(output_row);
    }
    result
}
