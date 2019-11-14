extern crate dragnit;
use dragnit::*;

fn main() {
    let schema = Schema::new(vec![
        Def::new("Point".to_owned(), DefKind::Struct, vec![
          Field {name: "x".to_owned(), type_id: TYPE_FLOAT, is_array: false, value: 0},
          Field {name: "y".to_owned(), type_id: TYPE_FLOAT, is_array: false, value: 0},
        ]),
      ]);

    let value = Value::decode(&schema, 0, &[126, 0, 0, 0, 126, 1, 0, 0]).unwrap();

    println!("{:?}",value);
}

