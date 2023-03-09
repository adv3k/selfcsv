use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::mem;

#[derive(Default)]
#[derive(Debug)]
struct CellData {
    index: String,
    data: String,
}

#[derive(Debug)]
struct Query {
    cell_id: String,
    formula: String,
    data: String,
}

impl Query {
    //new takes in the arguments, the header vec, and the celldata vec
    //new returns a Query instance, holding the cell id, the formula: RPL is default:
    fn new(
        args: &Vec<String>,
        cells: &Vec<CellData>,
        header: &Vec<String>,
    ) -> Result<Query, Box<dyn std::error::Error>> {
        match cells.iter().any(|x| x.index == args[1]) {
            true => {
                let cell_id = args[1].to_string();
                match ["SUM", "SUB", "MUL", "DIV"].iter().any(|&t| t == args[2]) {
                    true => {
                        let formula = args[2].to_string();
                        let mut data = String::new();
                        //find a way to have data be the rest of the args[3..] very frustrating
                        Ok(Query {
                            cell_id: cell_id,
                            formula: formula,
                            data: data,
                        })
                    }
                    false => Ok(Query {
                        cell_id: cell_id,
                        data: args[2].to_string(),
                        ..Default::default()
                    }),
                }
            }
            false => panic!("the cell you have chosen DNE"),
        }   
    }
}

//this creates the formula default of RPL (replace)
//as the default use of a cell is to replace the value
impl Default for Query {
    fn default() -> Self {
        Query {
            cell_id: String::new(),
            formula: String::from("RPL"),
            data: String::new(),
        }
    }
}
//this creates a default CellData instance to satisfy the _ arm of the match statement in fn main()
impl CellData {
    fn new() -> Self {
        Default::default()
    }
}


fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();

    println!("{:?}", args);

    let filepath = "";

    let line_vec: Vec<Vec<String>> = Vec::new();
    let line_vec: Vec<Vec<String>> = make_line_vec(filepath, line_vec).unwrap();

    let header: Vec<String> = line_vec[0].clone();

    let (cells, header_for_cells) = make_vec_cells(header, line_vec);

    if args[1] == "DISP".to_string() {
        display_cell_data(&cells, &header_for_cells);
    } else {
        let commands = Query::new(&args, &cells, &header_for_cells);
        println!("{:?}", commands.as_ref().unwrap());

        match commands.as_ref().unwrap().formula.as_str() {
            "RPL" => {
                let comm_cell_id = &commands.as_ref().unwrap().cell_id; let comm_data = &commands.as_ref().unwrap().data;
                replace(comm_cell_id.to_string(), comm_data.to_string(), cells, header_for_cells)
            },
            _ => Ok(vec![CellData::new()]),
        };
    }

    Ok(())
}

fn make_vec_cells(head_vec: Vec<String>, vect: Vec<Vec<String>>) -> (Vec<CellData>, Vec<String>) {
    let mut cell_vec: Vec<CellData> = Vec::new();
    let mut row_ind = 1;

    for vector in &vect[1..] {
        let mut ind = 0;

        for item in vector {
            let id = &head_vec[ind];
            ind += 1;
            let new_cell = CellData {
                index: id.to_string() + &row_ind.to_string(),
                data: item.to_string(),
            };
            cell_vec.push(new_cell);
        }
        row_ind += 1;
    }

    (cell_vec, head_vec)
}


fn make_line_vec(
    file: &str,
    mut vect: Vec<Vec<String>>,
) -> Result<Vec<Vec<String>>, std::io::Error> {
    let path = File::open(file)?;
    let reader = BufReader::new(path);

    let mut row_index = 0;

    for line in reader.lines() {
        let unwrapped_line = line
            .unwrap()
            .split(",")
            .map(str::to_string)
            .collect::<Vec<String>>();
        vect.push(unwrapped_line);

        if vect[row_index].len() > vect[0].len() || vect[0].len() > vect[row_index].len() {
            panic!(
                "\nRow: {} has too many elements\nShould only be {} elements\n",
                row_index,
                vect[0].len()
            );
        }

        row_index += 1;
    }

    Ok(vect)
}

//fn to display the cellData data field and the head_vec headers, seperated by \t
fn display_cell_data(
    cell_vector: &Vec<CellData>,
    head_labels: &Vec<String>,
) -> Result<(), std::io::Error> {
    let row_size = head_labels.len();
    let mut row_count = 1;

    for label in head_labels {
        print!("{:#}\t", label);
    }
    println!("\n");
    for cell in cell_vector {
        print!("{:#}\t", cell.data);
        row_count += 1;
        if row_count > row_size {
            row_count = 1;
            print!("\n\n");
        }
    }
    Ok(())
}


//takes in label(Query.cell_id), rpl data(Query.data), the vector of cells and the vector of header head_labels
//returns the updated cell vector
fn replace(
    label: String,
    rpl_data: String,
    mut cell_vector: Vec<CellData>,
    head_cells: Vec<String>
) -> Result<Vec<CellData>, Box<dyn std::error::Error>> {
    //get index of CellData struct that is put in
    let cell_index = cell_vector.iter().position(|i| i.index == label).unwrap();
    println!("{:?}", &cell_index);

    cell_vector[cell_index].data = rpl_data;

    display_cell_data(&cell_vector, &head_cells);

    Ok(cell_vector)
}
