fn run() {
    let mut counter = 0;
    let array = [87, 61, 56, 99, 30, 70, 12, 33, 44, 56, 34, 45, 50, 50];

    let iterator_end = array.len() - 1;
    
    
    
    for index in 0..iterator_end {
        if array[index] + array[index + 1] == 100 {
            counter += 1
        }
    }   
    
    println!("no of hundreds ; counter : {}", counter)

}