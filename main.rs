use std::io::{self, BufReader};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct CellData {
    index: String,
    data: String,
}

fn main() -> Result<(), std::io::Error> {
    let filepath = "D:\\Rust\\selfcsv\\dbcsv";

    let line_vec: Vec<Vec<String>> = Vec::new();
    let line_vec: Vec<Vec<String>> = make_line_vec(filepath, line_vec).unwrap();

    let header: Vec<String> = line_vec[0].clone();

    let (cells, header_for_cells) = make_vec_cells(header, line_vec);

    displayData(cells, header_for_cells);

    Ok(())
}

fn make_vec_cells(head_vec: Vec<String>, vect: Vec<Vec<String>>) -> (Vec<CellData>, Vec<String>) {
    let mut cell_vec: Vec<CellData> = Vec::new();

    for vector in &vect[1..] {
        let mut ind = 0;

        for item in vector {
            let id = &head_vec[ind];
            ind += 1;
            let new_cell = CellData { index: id.to_string() + &ind.to_string(), data: item.to_string() };
            println!("{:?}", new_cell);
            cell_vec.push(new_cell);
        }
    }

    (cell_vec, head_vec)
}


fn make_line_vec(file: &str, mut vect: Vec<Vec<String>>) -> Result<Vec<Vec<String>>, std::io::Error> {
    let path = File::open(file)?;
    let reader = BufReader::new(path);

    let mut row_index = 0;

    for line in reader.lines() {
        let unwrapped_line = line.unwrap().split(",").map(str::to_string).collect::<Vec<String>>();
        vect.push(unwrapped_line);

        if vect[row_index].len() > vect[0].len() {
            panic!("\nRow: {} has too many elements\nShould only be {} elements\n", row_index, vect[0].len());
        }

        row_index += 1;
    }

    println!("{:?}", vect);



    Ok(vect)
}


fn displayData(cell_vector: Vec<CellData>, head_labels: Vec<String>) -> Result<(), std::io::Error> {
    let row_size = head_labels.len();
    let mut row_count = 1;

    for label in head_labels {
        print!("{:#}\t", label);
    }
    println!();
    for cell in cell_vector {
        print!("{:#}\t", cell.data);
        row_count += 1;
        if row_count > row_size {
            row_count = 1;
            print!("\n");
        }
    }
    Ok(())
}
