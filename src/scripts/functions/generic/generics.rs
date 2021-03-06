//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// the algorithm, agnostic to data types
// T is a conventional name
fn largest<T: PartialOrd + Copy>(elements: &[T]) -> T {
    let mut largest = elements[0];
    for &element in elements.iter() {
        // T must be comparable, meaning it implements the comparison 
        // operator - this is a compile time requirement, just
        // like in C++
        // this requirement is defined as trait bounds
        // T: PartialOrd + Copy
        // PartialOrd: partial order - comparable, order-able 
        //          give the ability to use greater-than
        // Copy: to copy the value whenever possible
        if element > largest {
            largest = element;
        }
    }
    largest
}

// break the trait specification in a separate line
fn smallest<T>(elements: &[T]) -> T 
    where T: PartialOrd +
             Copy
{
    let mut smallest = elements[0];
    for &elem in elements.iter() {
        if elem < smallest {
            smallest = elem;
        }
    }
    smallest
}

fn test_largest() {
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
    // rust compiler can deduce the type of T, see the char
    // example
    let result = largest::<i32>(&nums);
    println!("largest i32: {}", result);

    let chars = vec!['t', 'h', 'e', 'r', 'e', 'i', 's'];
    let c = largest(&chars);
    println!("largest char: {}", c);

    println!("smallest char: {}", smallest::<char>(&chars));
}

fn main() {
    test_largest();
}



