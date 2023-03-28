mod data_structures;

fn main() {
    let getarray = data_structures::array::array(2);
    let getvector = data_structures::vector::vector(10);
    let gethashmap = data_structures::hashmap::hashmap();

    println!("Array: {:?}", getarray);
    println!("Vector: {:?}", getvector);
    println!("Hashmap: {:?}", gethashmap);

}
