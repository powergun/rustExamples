//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

fn consumer(_s: String) {
    ;
}

fn borrower(_s: &String) {
    ;
}

fn modifier(s: &mut String) {
    s.push('a');
}

fn demo_ownership() {
    // a owns the value
    let a = String::new();

    // b owns the value
    let b = a;

    // ownership passed to consumer()
    consumer(b);

    // error: value used here after move
    // consumer(b);
}

fn demo_immutable_borrowing() {
    // a owns the value
    let a = String::new();

    // a still owns the value
    // what is passed to the borrower is a reference
    borrower(&a);  // immutably borrowed
    borrower(&a);
    borrower(&a);

    // consumes a
    consumer(a);
}

fn demo_mutable_borrowing() {
    let mut a = String::new();
    
    // ---- this won't work, see fight with borrow checker ----
    // two immutable borrows, which live till the end of the scope
    // however a mutable borrow is also created via modifier()
    // rust disable the coexistance of immutable and mutable
    // borrow of the same data.
    // this helps to immediate data race
    // let _ref_1 = &a;
    // let _ref_2 = &a;
    
    // only one mutable borrow can exist at a time
    modifier(&mut a);

    borrower(&a);  // immutably borrow a
    consumer(a);

    // error: value used here after move
    // consumer(a);
}

fn main() {
    demo_ownership();
    demo_immutable_borrowing();
    demo_mutable_borrowing();
}