fn main() {
    let atwo = 2;
    let string = "hello";
    let charc = 'c';
    let half = 0.5;
    let mut name = "Fas"; // can have diff val
    let quit = false;
    let other_half = half; // 0.5 -> half -> other_half => other_half = 0.5
    let x = add(1,1); // 2
    let mut z = add(x,3);

    println!("rat is {:?}",z); // ! -> macro :? is for debug; for end user
                               // we can instead use {z} instead
    while (true) {
        if (z > 99) {
            println!("big rat");
            break;
        } else{
            z += 1;
            println!("eat more cheese");
        }
    }

    loop {
        if z > 1000 {
            break;
        }
        z += z;
        println!("{:?}", z);
    }

    let mut a = 0;
    while (a != 5){
        println!("{:?}",a);
        a += 1;
    }
}

fn add(a: i32, b: i32) -> i32 { 
    //params (a,b) return type -> i32
    return a + b;
}

