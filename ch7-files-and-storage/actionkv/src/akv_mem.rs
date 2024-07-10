use libactionkv::ActionKV;

///   Implements the 'Bitcask file format' for key-value storage:
///   |       fixed-width header        | variable-width key | variable-width value |
///   | checksum | key_len  | value_len |         key        |         value        |
///   | [] [] [] | [] [] [] | [] [] []  | [                ] | [                  ] |
///   |    u32   |   u32   |    u32     |   [u8; key_len]    |    [u8; value_len]   |

#[cfg(target_os = "windows")]
const USAGE: &str = "\
Usage:
    akv_mem.exe <FILE> get <KEY>
    akv_mem.exe <FILE> delete <KEY>
    akv_mem.exe <FILE> insert <KEY> <VALUE>
    akv_mem.exe <FILE> update <KEY> <VALUE>
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "\
Usage:
    akv_mem <FILE> get <KEY>
    akv_mem <FILE> delete <KEY>
    akv_mem <FILE> insert <KEY> <VALUE>
    akv_mem <FILE> update <KEY> <VALUE>
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut store = ActionKV::open(path).expect("failed to open file");

    match action {
        "get" => match store.get(key).unwrap() {
            None => eprintln!("{:?} not found", key),
            Some(value) => println!("{:?}", value)
        },

        "delete" => store.delete(key).unwrap(),

        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap();
        },

        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap();
        },

        _ => eprintln!("{}", &USAGE)
    }
}
