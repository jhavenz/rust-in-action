#[derive(Copy,Clone,Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Copy,Clone,Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

pub fn own_with_copy() {
    let sat_a = CubeSat { id: 0 };

    let a_status = check_status(sat_a); // sat_a is copied here
    println!("copied a: {:?}", a_status);

    let a_status = check_status(sat_a.clone()); // sat_a is cloned here
    println!("cloned a: {:?}", a_status);
}
