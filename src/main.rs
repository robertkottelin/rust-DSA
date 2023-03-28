mod data_structures;

fn main() {
    let getarray = data_structures::array::array(10);
    let getvector = data_structures::vector::vector(10);
    let gethashmap = data_structures::hashmap::hashmap();
    let heap = data_structures::binaryheap::binaryheap();

    println!("Array: {:?}", getarray);
    println!("Vector: {:?}", getvector);
    println!("Hashmap: {:?}", gethashmap);
    println!("Heap: {:?}", heap);
}
