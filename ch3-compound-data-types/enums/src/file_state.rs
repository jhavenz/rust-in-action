use std::fmt::{Display, Formatter};

#[allow(dead_code, unused_variables)]

#[derive(Debug,PartialEq)]
enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            FileState::Open => write!(f, "Open"),
            FileState::Closed => write!(f, "Closed"),
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

#[allow(dead_code)]
trait Read {
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl Read for File {
    #[allow(dead_code, unused_variables)]
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0)
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    #[allow(dead_code)]
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

pub fn run() {
    let mut f5 = File::new("5.txt");

    let mut buffer: Vec<u8> = vec![];

    if f5.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    f5 = open(f5).unwrap();
    let f5_length = f5.read(&mut buffer).unwrap();
    f5 = close(f5).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f5);
    println!("{} is {} bytes long", &f5.name, f5_length);
    println!("{}", text);

    let mut f6 = File::new("6.txt");
    let mut buffer6: Vec<u8> = vec![];
    f6 = open(f6).unwrap();
    let n_bytes = f6.read(&mut buffer6).unwrap();
    f6 = close(f6).unwrap();

    println!("{} byte(s) read from {:?}", n_bytes, f6);
    println!("{:?}", f6);
    println!("{}", f6);
}
