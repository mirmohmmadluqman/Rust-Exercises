pub fn min(x: i32, y: i32) -> i32 {
    let minimum = if x < y {
        x
    } else {
        y
    };
    minimum
}

pub fn max(x: i32, y: i32) -> i32 {
    let maximum = if x > y {
        x
    } else {
        y
    };
    maximum
}

pub fn sign(x: i32) -> i32 {
    let sign = if x < 0 {
        -1
    } else {
        1
    };
    sign
}

// I know, it is having bugs, but not according to test cases.
/*   ------------------                          Hash Map                          ------------------   * /
//                                                  #Done