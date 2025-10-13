pub fn mul(WETH: u32, ETH: u32) -> u32 { 
    let total = WETH * ETH;
    println!("Wrapped ETH: {WETH} * ETH: {ETH} = Total: {total}");
    total
}

pub fn div(WETH: u32, People: u32) -> u32 {
    let total = WETH / People;
    println!("Wrapped ETH: {WETH} given to {People} People = Total: {total}");
    total
}

/*         Completed         */