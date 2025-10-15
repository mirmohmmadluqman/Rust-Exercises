pub fn init(x: u32, y: u32, z: u32) -> Vec<u32> {
    let mut tensor: Vec<u32> = Vec::new();
    tensor.push(x); // or vec![x, y, z]
    tensor.push(y);
    tensor.push(z);
    tensor
}
