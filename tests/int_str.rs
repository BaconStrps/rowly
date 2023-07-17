use rowly::{
    column::{Column, VecColumn},
    table::{Table, VecTable},
};

#[test]
fn read_table() {
    let table = create_vec_table();
    let name_column = table
        .get_column("name")
        .data::<String>()
        .map(|s| format!("Hello, {}!", s));

    println!("{} names", name_column.clone().count());
    for name in name_column {
        eprintln!("{:?}", name);
    }

    let age_column = table
        .get_column("age")
        .data::<i32>()
        .filter(|x| **x % 2 == 0);

    println!("{} ages", age_column.clone().count());
    for age in age_column {
        eprintln!("{:?}", age);
    }

    let height_column = table.get_column("height").data::<f64>().map(|f| f * 2.0);

    println!("{} heights", height_column.clone().count());
    for height in height_column {
        eprintln!("{:?}", height);
    }

    let weight_column = table.get_column("weight").data::<f64>().map(|f| f * 2.0);

    println!("{} weights", weight_column.clone().count());
    for weight in weight_column {
        eprintln!("{:?}", weight);
    }
}

#[test]
fn test_rows() {
    let table = create_vec_table();
    let rows = table.rows();
    for row in rows {
        eprintln!("{:?}", row);
    }
    eprintln!("only jane");

    let rows = table
        .rows()
        .filter(|row| row.get("name") == Some(&"Jane".to_string()));

    for row in rows {
        eprintln!("{:?}", row);
    }
}

fn create_vec_table() -> VecTable {
    let mut table = VecTable::new();
    table
        .add_column(VecColumn::new("name", vec!["John", "Jane", "Joe"]))
        .add_column(VecColumn::new("age", vec![20, 21, 22]))
        .add_column(VecColumn::new("height", vec![1.8, 1.7, 1.9]))
        .add_column(VecColumn::new("weight", vec![80.0, 70.0, 90.0]));

    table
}
