fn main() {
    println!("Hello, world!");
    let mut x: i32 = 5;
    let _y: i32; //unused var, can stop the error by using _
    x += 3;
    let z: i32 = 5;
    {
        let x: i32 = 99;
        println!("the value of x is {} and the value of z is {}", x, z);
    }
    println!("the value of x is {} and the value of z is {}", x, z);
    def_x();
    break_x();
    tuples();
    assignments();
    numbers();
    chars();
    units();
    statements();
    ownership();
    assert_eq!(x,5); //macro -> !
    println!("success");
}


fn def_x() {
    let x: &str = "hello";
    println!("{}, world", x);
}

fn break_x(){
    let x: i32 = 1;
    let mut x = x;
    x = x + 3;
    let y = 4;
    let mut y = "test";
    println!("success");
    
}

fn tuples() {
    let (mut x, y) = (1, 2);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y,2);
    println!("success");
}

// Assigning multiple vars at once
// and also assigning their values despite them being in other structs
fn assignments() {
    let (x, y); // eqv to  let x; let y;
    (x, ..) = (3, 4);
    [.., y] = [1, 2];

    assert_eq!([x,y], [3, 2]);

    println!("Success");
}

fn numbers() {
    // arch based: isize/usize <-> T_size 
    // arch based -> word based, processors don't read 1 byte at a time
    // rather they read 1 word at a time
    // 32bit sys -> 4 bytes at a time (32 bits) at a time
    // 64bit sys -> 8 bytes at a time (64 bits) at a time
    // benefit of usize -> guarantees to always be big enough to hold
    // any pointer/offset in a ds
    // i -> integer (signed), u -> unsigned
    // f -> float

    // rustc will infer type if we don't explicitly assign type to a var
    let x: i32 = 50000000;
    let mut y = 5;
    y = x;
    // if we had explicitly gave y a type of u32, it wouldn't be
    // possible to assign x to y.
    let z = 10; // automatically assigned i32 type

    // we can however convert the int type to another:
    // however if it's much bigger than the type it's being converted
    // there'll be issues!!
    // also, in the line where I assign x, I can't go beyond the limits
    // of the type. otherwise compiler err.
    let v: i8 = x as i8;
    println!("{:?}",v);
    // rustc automatically tries to convert any int into i32, even if 
    // it can't fit
    let q: i128 = 1234123412342134123412341234123414;
    assert_eq!(i8::MAX, 127);

    let v1 = 256_u16 + 8; // you can mention type directly by: x_<type>;
    let v2 = i16::checked_add(251,8).unwrap(); // checked_add is basclly
                                               // the safer ver.
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597); 
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
    assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32); // these 2 are eqv.
    let mut sum1: i32 = 0;
    for i in -3..2 { //this is a how you create a range in rust
        sum1 += i;
    }

    assert!(sum1 == -5);
    sum1 = 0;
    for i in 0..=5 {
        sum1 += i;
    }
    assert!(sum1 == 15);

    for c in 'a'..='z' { // iterating through charc's, inclusive of z
        println!("{}", c);
        println!("{}", c as u8); // you can only <x>_u8 when assigning
    }

    assert!(1i32 - 2i32 == -1i32);
    assert!(1i8 - 2i8 == -1);
    assert!(1 - 2i32 == -1i32); //because by default i32 for int
    assert!(3 * 50 == 150);
    assert!(9.6_f32 / 3.2_f32 == 3.0_f32);
    assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32);
    assert!(24 % 5 == 4);

    // boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // bitwise ops.
    println!("0011 AND 0101 is {:04b}", 0b0011u8 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u8 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u8 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5); // to represent it as an int
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); // represent as hex
    // we use :04b to say show only a 4 bit value
    println!("{}", type_of(&q));
    println!("#Success");
}

fn chars() {
    let c1 = 'a';
    assert_eq!(std::mem::size_of_val(&c1), 4);

    let c2 = '~';
    assert_eq!(std::mem::size_of_val(&c2), 4); 
    let c3: char = 'x';
    print_char(c3);

    let _f:bool = false;
    let t:bool = true;
    if t {
        println!("{}", 's');
    }
}

fn print_char(c: char){
    println!("{}", c);
}

// what is a unit? basically if a fn doesn't return anything
// it by default returns a unit
fn units() {
    let _v: () = ();
    println!("{}",std::mem::size_of_val(&_v)); // size = 0
    let v: (i32, i32) = (2, 3);

    // the tuple and unit are not the same
    // and there is no difference in implicitly and 
    // explicitly returning a unit
    assert_eq!(_v, implicit_ret_unit());
    assert_eq!(_v, explicit_ret_unit());
    println!("Success!");
}

// Statements:
// instructs that perform some action but do not produce a value
// fn defs are statements as well as code that ends with ";" (mostly)
// Expression: Evaluate to a resultant val.
fn statements() {
    let x: u32 = 5;

    let y: u32 = {
        let x_squared = x * x;
        let x_cubed = x_squared * x;
        // This expr will be assigned to 'y'
        x_cubed + x_squared + x
    };

    let z: () = {
        // The ';'suppresses this expr. & a unit () is assigned
        // to 'z' instead.
        2 * x;
    };
    let v = {
        let mut x = 1;
        //x += 2 this will not work as this is considered an assignment
        // of x, and not an expr, i.e return ()
        // And in this case needs a ';' to finish the assignment
        // and then we call x without the ';' to make it an expr
        // so that v can evluate to x
        x += 2;
        x
    };
    println!("{}",v);
    let v1 = {
        let x = 3;
        x
    };
    assert_eq!(v1,3);

    let s = sum(1, 2);
    assert_eq!(s, 3);
    let (x1, y1) = (1, 2);
    let s1 = sum(x1, y1);
    assert_eq!(s, 3);
    //never_return();
    let b = true;
    let _v = match b {
        true => 1,
        false => {
            println!("Success!!");
            panic!("we have no value for 'false', but we can panic");
        }
    };
    println!("##Success!");
}

// diverging function -> a function that never returns to it's caller
fn never_return() -> ! {
    panic!() //sys panicks and causes err
    //unimplemented!()
    //todo!()
}

// the fn returns an i32 and not () because we didn't put a ';'
// in the last line, i.e it is the expr that sum will equate to
fn sum(a:i32, b:i32) -> i32 {
    a + b
}

fn implicit_ret_unit() {
    println!("returning ()");
}

    // we explicitly say we are returning a unit, unnecessary
fn explicit_ret_unit() -> () {
    println!("returning explicit ()");
}



fn type_of<T> (_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}






fn ownership() {
    //set of rules that govern mem management, enforced at compile time
    // Rules:
    // 1. each val has an owner
    // 2. there can only be 1 owner at a time
    // 3. when the owner goes out of scope, the value will be dropped
    // What is scope? range within a prog, where an item is valid
    // global scope -> accessible throughout the prog
    // local scope -> acessible only within particular fn or block
    //             -> not accessible outside that
    // This is based on memory alloc. i.e which region of RAM is used
    // by the program at runtime: stack or heap
    
    // stack and heap aren't inherently better than each other, they
    // are both part of RAM, and all access times on ram are the same
    // what distinguishes them is the way they are used.
    // Heap is like a public park, anyone can come to use it and store 
    // their stuff if there's space, and if you keep taking stuff to 
    // the park without ever picking it up. You are "leaking" your 
    // resources
    // Stack is like a hotel room, way smaller than a park, but you
    // know you will have space when you arrive, and you can put
    // whatever you want there, and when you leave. It gets cleared out! 
    // Stack:
    // Last In, First Out LIFO
    // All data stored on stack must have a known fixed size (int ..)
    // pushing to the stack is faster than alloc. on heap because
    // the location of the new data is always at the top of stack
    // types of unknown size will get allocated to the heap and a
    // pointer to the value is pushed to the stack, because a pointer
    // has a fixed size (usize)


    // Heap:
    // data is no known fixed sized belongs to the heap
    // alloc'ing data on the heap will return a pointer (addr to locat.)
    // alloc'ing on the heap is slower than pushing to stack
    // accessing data on the heap is slower, not because of access time
    // but because it has to be accessed using a pointer which points
    // to an address


    // s is not valid here, it's not yet declared
    let _s = "hello"; // s is valid from this point forward
    
    // String type -> string is mutable, string size can change at
    // run time, string's pointer is stored on the stack with the
    // string itself on the heap
    // val. of string is stored on the heap

    let s1 = String::from("hello");

    // s1:
    //  STACK                         HEAP
    // name |   val                 index | val
    // ptr  |    -------------------->0   | h
    // len  |   5                     1   | e
    // cap. |   5                     2   | l
    //                                3   | l
    //                                4   | o
    // size of s1 will be size_of(ptr)+size_of(len)+size_of(cap.)
    // ==> (usize) + (usize) + (usize)
    // ==> 8 + 8 + 8
    // ==> 24
    // ptr is a pointer to data stored on the heap
    // s1 is just a var that points to the mem addr on the heap
    // with the number of index the data is stored in
    // i.e s1.ptr...s1.ptr+s1.len == 0..4 mem addr
    // I don't know what capacity is yet. TBD

    // Copy v/s Move
    // copy is cheap, scalar vals with fixed size will automatically get
    // copied din the stack, copying here is cheap,
    // but copying dynamically sized data is too expensive, so we
    // instead move it. As the "move" op. transfers ownership of the 
    // dta pointer rather than duplicating the entire dataset
    // And copy/move are equally fast for both fixed size data types,
    // however copy gives much more flexibility to what we can do
    // whereas moving offers no performance benefit and leaves the
    // original value unusuable.

    let x = 5; // default i32, and i32 -> fixed size
    let y = x; // here y is assigned x, and since x is i32 (fixed size)
               // instead of moving the value/etc. it instead copies it
               // and gives it y.
    
    let s1 = String::from("hello"); // s1 is just a ptr to data on heap
    
    println!("{:?}",std::ptr::addr_of!(s1));
    
    let s2 = s1; // since ptr+len+cap. are all fixed size (usize)
                 // total = 3*(usize)
                 // it will be copied instead of moved.
                 // and since s1 and s2 point to the same memory loc.
                 // in heap, this violates the 2nd rule of rust mem.
                 // where there can only be 1 OWNER at a time
                 // hence s1 will be dropped and cannot be used after
                 // assigning it to s2, to avoid dangling pointers.
 
    // unlike the i32 x and y, ptr s1 and s2 had one of them eliminated
    // as they pointed to the same memory addr in the heap.
    // you can still use x even after you assign y to it, as it just
    // copies the value of x (i32)

    // what if you wanted two same strings
    // Deep copy
 
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!(
        "s1 = moved, s2 = {:?}, s3 = {:?}, s4 = {:?}", 
        std::ptr::addr_of!(s2), std::ptr::addr_of!(s3), std::ptr::addr_of!(s4)
    );
    let s5 = String::from("byebye"); // s5 comes into scope
    println!("{}",type_of(&s5));
    println!("{:?}", std::ptr::addr_of!(s5));
    takes_ownership(s5); // s5's value moves into the func... and so is
                         // no longer valid here
                         // roughly it's the same thing as when we assign s5
                         // and now because there are 2 pointers to the same
                         // heap addr, so s5 is eliminated so that there is 
                         // only 1 owner of the mem. addr

    let x = 5;           // x comes into scope
    makes_copy(x);       // x would move into the function, but i32 is instead
                         // copied, so we can still use x afterwards


    let s6 = gives_ownership(); // fn gives_ownership moves it's return val
                                // into s6

    let s7 = String::from("hi");

    let s8 = take_and_give(s7); // here s7 loses it's ownership of the string,
                                // which then goes to the fn take_and_give()
                                // which then evaluates itself to be an expr.
                                // that is then assigned to s8
                                // i.e s7->take_and_give()->s8


    // Ownership prevents mem. safety issues:
    // 1. dangling pointers
    // 2. double-free -> freeing freed mem.
    // 3. mem. leaks -> not freeing mem. that should've been freed
    {
        let x:String = String::from("hello");
        let y:String = x.clone();
        println!("{},{}", x, y);
    };

    {
        let x:String = String::from("rat");
        let y:String = take_and_give(x);
        println!("{}", y);
    };

    {
        let x:String = String::from("world");
        let y = x.as_bytes();          // into_bytes() converts String into array
                                       // of bytes, however it consumes x i.e we 
                                       // cannot use it anymore, instead we can
                                       // use as_bytes() instead as it uses
                                       // references TBD
        println!("{:?}", y);
    };

    {
        let x:String = String::from("world");
        print_str(x.clone()); //we use clone
        println!("{:?}", x);
        
        fn print_str(s:String) {
            println!("{}",s)
        }
    };

    {
        let x0:(i32,i32,(),String) = (1,2,(),"hello".to_string());
        // there is a difference b/w strings and string literals, i.e
        // when you assign a string to something, you are actually just
        // clone the ptr to the heap mem addr, the len of the string,
        // and the capacity (TBD). and there can only be 1 ptr to a mem addr.
        // i.e there can only be 1 owner of a mem addr. so what happens
        // is that the x0's string's ownership get's taken away 
        let x:(i32,i32,(), &str) = (1,2,(),"hello"); 
        // a string literal however is simpler than that, and is stored
        // on the stack.because it's immutable and hence it's size is known
        // at compile time.
        let y:(i32,i32,(),&str)  = x; // without using .clone
        let y0:(i32,i32,(),String) = x0.clone(); 
        println!("{:?}", x0); 
        println!("{:?}", x); 
        println!("{:?}", y0); 
        println!("{:?}", y); 
        
        
    };

    {
        let s: String = String::from("hello");
        let mut s1 = s; // we can change the mutuability of a var
                        // after ownership is transferred 
        s1.push_str(" world");
        println!("{:?}",s1);
    }
    

} // scope of all vars declared ends 


fn takes_ownership(some_str: String) { //some_str comes into scope
    println!("{:?}", std::ptr::addr_of!(some_str));
    println!("{}", some_str);
} // Here, some_str goes out of scope and 'drop' is called. The backing
  // mem. is freed. i.e the string has been de-allocated, and doesn't
  // exist after the func. returns. If we wanted the string to be valid
  // after, all we have to do is change some_str: String to
  // some_str : &String, because now we are borrowing it instead of
  // taking ownership (TBD), however this method isn't the best way
  // to do it, the better way would be just to take a string slice (TBD)
  // i.e some_str : &str, and we call it like takes_ownership(&s5)
  // currently this fn implicitly returns a unit i.e () 
fn makes_copy(some_int: i32) {
    println!("{}", some_int); 
}


fn gives_ownership() -> String {
    let some = String::from("hi");
    some // this is now an expression
}

fn take_and_give(some_string: String) -> String {
    some_string 
}








