#![allow(unused_variables)]
use rand::prelude::*;

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        File {
            name: String::from(name),
            data: data.clone(),
        }
    }

    fn read(self: &mut File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        Ok(read_length)
    }
}

fn open(f: File) -> Result<File, String> {
    if random() && random() && random() {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }

    Ok(f)
}

fn close(f: File) -> Result<File, String> {
    if random() && random() && random() {
        let err_msg = String::from("Interrupted by signal");
        return Err(err_msg);
    }

    Ok(f)
}



pub fn run() {
    let f1 = File::new("f1.txt");

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);

    let mut f2 = File::new("f2.txt");

    let mut buffer2: Vec<u8> = vec![];

    f2 = open(f2).unwrap();
    let f2_length = f2.read(&mut buffer2).unwrap();
    f2 = close(f2).unwrap();

    let text = String::from_utf8_lossy(&buffer2);

    println!("{:?}", f2);
    println!("{} is {} bytes long", f2.name, f2_length);
    println!("{} contains: {}", f2.name, text);

    let mut f3 = File::new_with_data("f3.txt", &vec![114, 117, 115, 116, 33]);

    let mut buffer3: Vec<u8> = vec![];

    f3 = open(f3).unwrap();
    let f3_length = f3.read(&mut buffer3).unwrap();
    f3 = close(f3).unwrap();

    let text = String::from_utf8_lossy(&buffer3);

    println!("{:?}", f3);
    println!("{} is {} bytes long", f3.name, f3_length);
    println!("{} contains: {}", f3.name, text);

    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f4 = File::new_with_data("f4.txt", &f4_data);

    let mut buffer4: Vec<u8> = vec![];

    f4 = open(f4).unwrap();
    let f4_length = f4.read(&mut buffer4).unwrap();
    f4 = close(f4).unwrap();

    let text = String::from_utf8_lossy(&buffer4);

    println!("{:?}", f4);
    println!("{} is {} bytes long", f4.name, f4_length);
    println!("{} contains: {}", f4.name, text);
}
