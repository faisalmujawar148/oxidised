use rand::Rng;


// This is a binary create because we have a main fn here.
// If we didn't have a main(), this would be a library crate!
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
    borrowing();
    compound_types_strings(); // focusing on Strings and &str, arrays, slices, tuples, structs, enums
    compound_types_arrays();
    compound_types_slices();
    compound_types_tuples();
    compound_types_structs();
    compound_types_enums();
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


fn size_of<T> (_: &T) -> usize {
    std::mem::size_of::<T>()
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
    // I don't know what capacity is yet.
    // Cap is basically the max len, and if the string grows the values
    // within the stack & heap get updated.
    // Cases:
    //      1. len < cap:
    //          here the len of the string hasn't outgrown the cap
    //          so the only thing that gets updated in the stack
    //          is the len! ptr and cap stay the same
    //      2. len >= cap:
    //          here the len of the string has outgrown/matched the
    //          cap, so the compiler allocates a new larger buffer
    //          on the heap for the string, all the old data is 
    //          copied over and the old buffer is freed
    //          All the three stack fields get updated: ptr to the 
    //          new mem. addr, len of the string, cap is doubled
    //          from the previous val, and if that isn't enough it
    //          will be eqv to the length of the string + a bit more
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

    {
        let x: Box<i32> = Box::new(5); // allows for direct alloc. to heap, even if
                                       // type is usually pushed to stack mem.
                                       // i.e x will be a ptr to mem addr of the
                                       // i32 in the heap
        let mut y: Box<i32> = Box::new(1);

        *y = 4; // we aren't don't y = 4, because y is a ptr, y* = value at mem add.
        

        assert_eq!(*x, 5);
        println!("Success!");
    };

    {   
        struct Person {
            name: String,
            age: Box<u8>,
        } // you can (in most cases), omit ';' when it ends with a '}'
        
        let person: Person = Person {
            name: String::from("Alice"),
            age: Box::new(20),
        };
        // 'name' is moved out of person -> because of the mem rule of only 1
        // owner of mem., but 'age' is referenced, i.e now we can't use the var
        // person anymore
        let Person {name, ref age} = person;
        
        println!("The person's age: {}, name: {}", age, name);
        
        // println!("{:?}", person);
        // Error! borrow of partially moved value
        
        // println!("The person's name from the person struct: {}", person.name);
        // isn't possible as the ownership of the mem. addr of 'name' gets changed
        // 'person' cannot be used, but person.age can be as it is not moved.
        println!("The person's age from the person struct: {}", person.age);


    };

    {   
        struct Person {
            name: String,
            age: Box<u8>,
        } // you can (in most cases), omit ';' when it ends with a '}'

        
        let person: Person = Person {
            name: String::from("Alice"),
            age: Box::new(20),
        };
        // Experimenting to check what happens if I use ref on 'name'
        // will it allow me to preserve the struct? Yes, because I'm referencing
        // the mem addr. i.e I'm not taking ownership, I'm only borrowing it!
        // 'name' is moved out of person -> because of the mem rule of only 1
        // owner of mem., but 'age' is referenced, i.e now we can't use the var
        // person anymore
        let Person {ref name, ref age} = person;
        
        println!("The person's age: {}, name: {}", age, name);
        
        // println!("{:?}", person);
        // Error! borrow of partially moved value
        
        println!("The person's name from the person struct: {}", person.name);
        // 'person' cannot be used, but person.age can be as it is not moved.
        println!("The person's age from the person struct: {}", person.age);


    };

    {
        let t: (String, String) = (String::from("hello"), String::from("world"));

        let _s: String = t.0; // now _s is the owner of t.0, we can't
                              // access t anymore, but we can access
                              // some of the partial elements 
                              // still not moved.


        println!("{:?}",t.1);
    };

    {
        let t: (String, String) = (String::from("hello"), String::from("world"));

        let (ref s1, ref s2) = t;

        println!("{:?},{:?},{:?}", s1, s2, t);
    };
    
    {
        let t: (String, String) = (String::from("hello"), String::from("world"));

        let (s1, s2) = t.clone(); // alt. method, where we deconstruct a clone of
                                  // t instead making references to the mem. addr.
                                  // within the struct.

        println!("{:?},{:?},{:?}", s1, s2, t);
    };

    

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

// Temporarily acessessing data w/out ownership is borrowing
// when borrowing you're taking a reference (pointer) to the data
// not the data itself.
// Rules: 
// 1. You can either have 1 mutable reference or x number of immutable refs.
// 2. Refs must always be valid. (?)


// Ref. structure:
//              s                   s1                heap
//              -                   -                   -
//      name    |   val     name    |   val     index   |   val
//      ptr     |   -------->ptr    |   --------->0     |   h
//                           len    |   5         1     |   e
//                           cap.   |   5         2     |   l
//                                                3     |   l
//                                                4     |   o
//  let s1: String = String::from("hello");
//  let len = calculate_length(&s1);
//  fn calculate_length(s: &String) -> usize {
//      s.len()
//  };
//  now we can still use s1, as the ownership of the mem. addr is still
//  under s1, if we had passed 's1' instead of '&s1' the ownership of the
//  mem. addr would have gone to len -> calculate_length  
fn borrowing() {
        {
            let mut x: i32;
        };

        // s1 still has ownership over the mem addr. in the heap
        // so we can still call it without the compiler complaining
        // we aren't taking ownership of the value/transferring
        // we are instead borrowing the value through referencing
        {
            let s1: String = String::from("hello");
            let len: usize = calculate_length(&s1);
            fn calculate_length(s: &String) -> usize {
                s.len()
            };
            println!("{:?}, {:?}", s1, len);
        };
        
        {
            let s1: String = String::from("hello");
            let len = calculate_length(s1);
            fn calculate_length(s: String) -> usize {
                s.len()
            };
            println!("{:?}", len);
        };
        
        {
            let mut s1: String = String::from("hello");
            change(&mut s1);
            fn change(s: &mut String)  {
                s.push_str(", world") 
            };
            println!("{:?}", s1); // there can only be either 1
                                                 // mutable ref, or multiple
                                                 // immutable refs. not both
        };
        
        {
            let mut s1: String = String::from("hello");
            let s2: String = change(&mut s1);
            fn change(s: &mut String) -> String {
                s.push_str(", rat");
                s.to_string() 
            }; // I seem to confusing ownership & borrow
            println!("{:?}", s2); // there can only be either 1
                                  // mutable ref, or multiple
                                  // immutable refs. not both
        };

        // borrow rules:
        // 1. You can have any number of inmutable refs
        //    XOR have 1 mutable ref
        // 2. All references must be valid
       
        {
            let mut s: String = String::from("hello");
            let s1 = &s; // non-mutative ref
            let s2 = &s; // another ^
            println!("{:?},{:?}", s1, s2);
            let s3 = &mut s; // this won't automatically break the compiler!
                             // because we aren't calling it immediately
                             // in fact we can use it after we use a 
                             // function that takes ownership of s1/s2
                             // i.e removes the references so that only 
                             // 1 mutative reference remains

            fn change(s: &mut String) {
                s.push_str(", bye");
            };
            change(s3);
            println!("{:?}", s3);
        };


        // dangling refs.

        {
            let s1: String = dangle(); // if we still were returning &s, this
                                       // would cause the compiler to complain
                                       // as we need it to be a valid reference
                                       // but once the string within the fn
                                       // reaches the '}' it will be out-of-scope
                                       // so the reference will not be valid
                                       // outside of the fn
                                       // that is why instead we just pass
                                       // the string itself, so that s1
                                       // becomes the owner of the string 's'
            fn dangle() -> String {
                let s = String::from("hello");
                // &s; what happens here is that the string after reaching the
                // end of the scope i.e '}' is that the reference to it will be 
                // dropped. i.e the reference will no longer be valid!!
                // the compiler will complain about it
 
                s
            }
            println!("{:?}",s1);
        };

        {
            let x: i32 = 5;
            let p: &i32 = &x;
            println!("mem addr of x is {:p}", p);
        };


        {
            let mut s = String::from("hello");
            borrow_obj(&s);
            println!("Success");
    
            fn borrow_obj(s: &String) {}
        };

        {
            let mut s: String = String::from("hello, ");
            push_str(&mut s);
            println!("Success! {}", s);
    
            fn push_str(s: &mut String) {
                s.push_str("world");
            }
        };


        {
            let mut s: String = String::from("hello");
            let p: &mut String = &mut s;
            p.push_str(", test");
            println!("{}",p);
        };

        {
            // ref can be used to take references to a val
            // similar to &

            let c: char = 'a';
            let r1: &char = &c;
            let ref r2 = c; // we don't have to say ref r2 = &c
            // under the hood ref and & are in essence the same
            // however the way the compiler/syntax uses them
            // differs, TBD I know that they are useful
            // when you are dealing with Options
            assert_eq!(*r1, *r2);
            assert_eq!(get_addr(r1),get_addr(r2));
            println!("{} {}",get_addr(r1),get_addr(r2));
            println!("Success");
        };

        {
            let mut s: String = String::from("hello");

            let r1 = &s;
            let ref r2 = s;

            println!("{} {}", r1, r2);
        };

        {
            let mut s: String = String::from("hello");
            let r1 = &mut s;
            r1.push_str("world");
            println!("{}", r1);
            let r2 = &mut s;
            r2.push_str("!");

            println!("{}", r2); // this is valid because we destroy r1
                                // before we use r2
        };

        {
            let mut s: String = String::from("hello");
            let r1 = &mut s;
            r1.push_str(" rat");
            let r2 = &mut s;
            // no errors because we aren't calling them at the same
            // time, in fact
            r2.push_str("!");
            println!("{}", r2); // is valid
            // only println!("{} {}", r1, r2); would cause any errors
        }
}


fn compound_types_strings() {
    // String vs &str
    // String is heap allocated, and owns its' contents, i.e
    // the mem addr + capacity is under it's ownership, and is mutable
    // String is also made up of char
    //
    // &str <-> string slice is stack allocated. and doesn't own its' own data
    // and it is immutable, &str is basically a view on a sequence of characters
    // (stored as utf-8 bytes) in mem.
    // .
    // use &str if you want to view of a string, as it's more lightweight + eff.
    // use String if you need to mutate the string and own the data
    // .
    // String literal(&str) is a sequence of chars enclosed in -> "
    // Fixed size, compile-time known sequence of utf-8 bytes
    // the type is '& static str', which means the data is stored in static
    // storage, meaning it is valid throughout the entire lifetime of the prog.
    // the data is hardcoded into the exec and stored in read-only mem.,
    // i.e they are immutable
    // .
    // String slices only hold a reference to the data allocated on the
    // heap/stack, hence they are effecient, also they are immutable
    // 
    // String Literal (&str):
    //          STACK                         HEAP --> wrong, actually BINARY 
    //      name    |   value           index   |   value
    //      ptr     |   ----------------> 0     |   h
    //      len     |   5                 1     |   e
    //                                    2     |   l
    //                                    3     |   l
    //                                    4     |   o
    // String:
    //  STACK                         HEAP
    // name |   val                 index | val
    // ptr  |    -------------------->0   | h
    // len  |   11                    1   | e
    // cap. |   11                    2   | l
    //                                3   | l
    // String Slice (&str):           4   | o
    // name |   val                   5   | ' '
    // ptr  |    -------------------->6   | w
    // len  |   11                    7   | o
    //                                8   | r
    //                                9   | l
    //                                10  | d
    // let s: String = String::from("hello world");
    // let slice = &s[6..11] <- remember that if there is no = it is exclusive
    // .
    // String literal.datatype == string slice.datatype 
    {
        let s: &str = "hello"; // We can't use str directly as it is dynamically
                               // sized type (DST), however a reference is
                               // always going to be a fixed size (usize)
                               // and the rust compiler needs everything to have
                               // a fixed size, so the main way we use str
                               // is through a reference to it

        
        println!("Success");
    }

    // We can also use str by boxing it

    {
        // let s: Box<str> = Box::new("hello, world"); isn't possible because
        // "hello, world" is a &str, so we first have to convert it

        let s0: Box<str> = "hello, world".into();// this is the abstracted
                                                 // way of doing it, it takes
                                                 // the type based on the 
                                                 // var it is being assigned to
                                                 // and based on that, converting
                                                 // it is super good
                                                 // but hides some learning
                                                 // chances

        let s: Box<str> = Box::from("hello, world");
        // this is super cool, I just kinda understood what ::from means
        // it means that instead of taking it from the stack first 
        // like ::new(val), which takes val first and then moves to the
        // heap -> impossible for a DST (dynamically sized type) like str
        // it will instead take it directly from the heap, bypassing
        // the need to size it from the stack!!
        // implication for String::from, and implies String::new!!
        // what the above does under the hood:

        let s1: Box<str> = "hello, world".to_string().into_boxed_str();
        println!("{}",s);
        println!("{}",s1);
    }


    {
        let s: &str = "hello, world";

        fn greets(s: &str) {
            println!("{}",s)
        }
        greets(s);
    }
    
    {
        let s: Box<str> = "hello, world".into();
        fn greets(s: &str) {
            println!("{}",s)
        }
        greets(&s);
    }
    // I just understood what &str -> String slices & String literals mean
    // String Literals are literally stored as a read-only file (binary)
    // so the &str is literally:
    // String Literal (&str):
    //          STACK                        BINARY 
    //      name    |   value           index   |   value
    //      ptr     |   ----------------> 0     |   h
    //      len     |   5                 1     |   e
    //                                    2     |   l
    //                                    3     |   l
    //                                    4     |   o 
    // let s:&str = "hello";
    // String Slice (&str): 
    //          STACK                          HEAP  
    //                                   index   |   value
    //                                     0     |   h
    //                                     1     |   e
    //      name    |   value             ...    |  ...
    //      ptr     |   ----------------> 10     |   w
    //      len     |   5                 11     |   o
    //                                    12     |   r
    //                                    13     |   l
    //                                    14     |   d
    // let bigString: String = String::from("hello ratss world");
    // let stringslice: &str = &bigString[10...=14];
    //
    // &str is literally a string view!!! i.e regardless of where it is stored
    // it is just a ptr + len, it will always just point to where the callers'
    // data lives
    // i.e this is also why it is preferred over &String, because it can accept
    // any type, while &String is only limited to heap, plus &String is a
    // ref to ref to heap mem. addr.
    // .
    //              s(&String)         s1(String)        heap
    //              -                   -                   -
    //      name    |   val     name    |   val     index   |   val
    //      ptr     |   -------->ptr    |   --------->0     |   h
    //                           len    |   5         1     |   e
    //                           cap.   |   5         2     |   l
    //                                                3     |   l
    //                                                4     |   o
    // String type is stored as a vector of bytes (Vec), but guaranteed to
    // always be a valid utf-8 seq, it is heap allocated, growable, and not
    // null terminated.
    {
        let mut s: String = String::from("hello");
        s.push_str(" squeaks");
        s.push('!');
        println!("{}",s);
    }

    {
        let mut s: String = String::from("hello");
        s.push(','); // char gets copied as it's only 4 byte
        s.push_str(" world");
        s += "!"; //&str for borrowing
        s += "@"; //&str
        println!("{}",s);
    }

    {
        let s: String = String::from("hello rats");
        let s1: String = s.replace("rats", "cats");
        // string.replace(x,y) borrows s, creates a brand new String
        // on the heap with the changes, returns the ownership the new
        // string to s1, s is completely untouched
        // i.e we can still use the original string without issues!!

        

        println!("{}",s1);
        println!("{}",s);
    }

    {
        let s1: String = String::from("hello");
        let s2: String = String::from("world!");
        let s3 = s1.clone() + s2.as_str();// concatenation: String + &str, 
                                          // can't do String + String. The first arg
                                          // has to be a String, and the other following
                                          // args have to be string slices
        let s4 = s1 + &s2; // another way of using a String as str, is it make it a
                           // reference. then it'll be treated as a string slice &str
                           // however there is some magic involved here, because it 
                           // shouldn't be &str it should be &String, right?
                           // the compiler however has a deref (TBD) method that 
                           // basically converts &String -> &str
                           // &String -> String -> str -> &str
        assert_eq!(s3, "helloworld!");
        // println!("{}",s1); we can no longer use this as it's was taken
        println!("{}", s3);
    }
    

    // Converting &str to String
    {
        let s: &str = "hello world";
        greets(s.to_string());

        fn greets(s: String) {
            println!("{}",s)
        }
    }
    
    {
        let s: &str = "hello world";
        greets(String::from(s));

        fn greets(s: String) {
            println!("{}",s)
        }
    }

    // Hexadecimal values / Unicode code points

    {
        let byte_esp = "I'm writing Ru\x73\x74!";
        let uni_code = "\u{211D}";

        println!("{}", byte_esp);
        println!("{}", uni_code);
    }

    // Raw String -> string without escape sequences
    {
        let raw_str = r"Escapes don't work here: \x73 \u{211D}";

        // but if you need quotes

        let quote = r#"And then "rats"!"#;

        println!("{}{}", raw_str, quote);
    }

    // String indexing
    {
        let s1: String = String::from("hello world");
        // Method 1: iterate over chars safetly using .nth() <- iterator
        // .chars() converts the string into a list/iterator of chars,
        // and then we use .nth which will just walk over the iterator and return
        // whichever index we are looking for, it returns it as Option<char>,
        // so we have to unwrap it if we want to use the val.
  
        let s0 = s1.chars().nth(0);
        println!("{:?}",s0);
        println!("{:?}", type_of(&s0));
        println!("{}", s1); // Since String is heap alloc. we don't move it, we borrow
                            // and then copy the values, etc. etc. so we can still use
                            // it

        let proper_s0 = &s0.unwrap();

        println!("{}", proper_s0);

        // Method2: Convert to byte_stream? and then directly index over that
        // .as_bytes(), and then index it.

        let s2 = s1.as_bytes()[0];
        println!("{}",s2 as char); // on it's own it will return the byte val of it, so have to
                                   // tell our print macro to convert it to a char
        // Method 3: convert to &str slice, because String implements Index<Range<usize>>,
        // not Index<usize> directly, so even though [0..1] will only return 0,
        // the params requires a range to be provided
        let h = &s1[0..1];


        println!("{:?}",h);
        // Note: Unicode chars can take 3/4 bytes, eg:'‿' is 3 bytes, so the range we have to iterate
        // is actually like

        let note: String = String::from("hello,‿,rats");

        let h1 = &note[6..9];

        println!("{}",h1);
        
        // Which is why using the iterator method is safer, and is the preferred way
    }

    {
        for c in "起初我們並不熟悉，漸漸地就熟悉起來了。".chars() {
            println!("{}", c);
        };
    };

}

// Arrays:
// Fixed size collection of the same data type, stored as contiguous block in stack mem.
// syntax: [T; Length] -> indicating that length is fixed at compile time.
// Arrays neither shrink nor grow, they retain their size.
fn compound_types_arrays() {
    {
        let arr_rats:[i32; 5] = [1, 2, 3, 4, 5];
        assert!(arr_rats.len() == 5);
    
        println!("5 rats!!");
    }

    {
        // We also let the compiler infer the type for us

        let test_arr:[_; 2] = [1, 2];
        let test_arr:[i32; _] = [1, 2];
        let test_arr2 = ['r', 'a', 't'];
        println!("{:?}", size_of(&test_arr));
        assert!(size_of(&test_arr) == 8); // because each i32, is 4 bytes, and 2 of them = 8
        println!("{:?}", size_of(&test_arr2)); // each char is 4 bytes, and 3 of them = 12
    }

    {
        let list: [i32; 100] = [1; 100]; // [1,1,...,1]
        let mut string: String = String::from(""); 
        for x in list {
            string += &x.to_string();
        }
        println!("{}",string);
    }


    {
        let names: [String; 2] = [String::from("Rat"), String::from("Mousey")];

        let name0 = names.get(0).unwrap(); // safer than direct indexing as it gets returned
                                           // as an Option<String>, which will return None if
                                           // it can't find the value at the index mentioned
                                           // super useful when doing this at runtime

        let ref name1 = names[1]; // doesn't have safety built in, if the value isn't found
                              // runtime panic. [index out of bounds]
                              // Also, you can't directly use it, as the array contains type String
                              // which doesn't implement the 'copy' trait, i.e can only 'move'
                              // and if we do that we break the array. i.e we have to ref names
        let name2 = &names[1];
        println!("{}{}{}",name0,name1,name2);
    }
    
}
// Slices:
// Reference to a contiguous sequence of elements in a collection
// provide a way to borrow part of the collection without taking full ownership
// can be created from arrays, vectors, Strings, and other collections implementing the 
// Deref trait (TBD)
// we already saw string slice &str[1..3]
// it's the same data struct as a &str slice, i.e 
// String Slice (&str): 
//          STACK                          HEAP  
//                                   index   |   value
//                                     0     |   h
//                                     1     |   e
//      name    |   value             ...    |  ...
//   ptr(usize) |   ----------------> 10     |   w
//   len(usize) |   5                 11     |   o
//                                    12     |   r
//                                    13     |   l
//                                    14     |   d
// Slices is a just a reference to ANY contiguous sequence of elem.s in a collection
// i.e the data could be in heap, binary, stack, etc. it doesn't matter
// eg:
// String Literal Slice (&str)
//          STACK                          BINARY
//                                   index   |   value
//                                     0     |   h
//                                     1     |   e
//      name    |   value             ...    |  ...
//   ptr(usize) |   ----------------> 10     |   w
//   len(usize) |   5                 11     |   o
//                                    12     |   r
//                                    13     |   l
//                                    14     |   d
// Here is the slice is of a string that's stored in the binary
// Fixed size Array Slice (&arr[10..15])
//          STACK                          STACK
//                                   index   |   value
//                                     0     |   h
//                                     1     |   e
//      name    |   value             ...    |  ...
//   ptr(usize) |   ----------------> 10     |   w
//   len(usize) |   5                 11     |   o
//                                    12     |   r
//                                    13     |   l
//                                    14     |   d
// if it was a Dynamic size Array i.e Vec, it would be allocated on the HEAP
// instead
fn compound_types_slices() {
    {
        let arr = [1,2,3];
        let s1: &[i32] = &arr[0..2];
    };
    // [i32] and str are slice types, but we can't directly use them as the are DST,
    // and the compiler will complain if they size isn't known i.e we have to use a 
    // ref to it

    {
        let s2: &str = "hello, world" as &str; // We can't directly use str, (mentioned ^)
        println!("{}",s2);
    };

    {

        let arr: [char; 3] = ['r','a','t'];
        let slice: &[char] = &arr[..3]; 
        // 0..2 (with 2 not being included)
        // slice & arr
        // remember &[char] is the ptr (usize) + len (usize) 
        println!("{:?}", std::mem::size_of_val(&arr)); // because 3 * char
        println!("{:?}", std::mem::size_of_val(&slice)); // because ptr(usize) + len(usize)
        println!("hello world");
    }

    {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        let sice: &[i32] = &arr[1..4];
        println!("{:?}",sice);
    }

    {
        let s = "你好，世界";
        let slice = &s[0..3]; //unicode characters take 3 bytes
        assert!(slice == "你");
        let slice = &s[0..6]; //unicode characters take 3 bytes, so 2 of them will be 6
        println!("{}", slice);
    }

    {
        let mut s = String::from("hello world");
        
        // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
        // It works because `&String` implicitly converted to &str.
        // This is called Deref coercion TBD

        let letter = first_letter(&s);

        println!("the first letter is: {}", letter);
        
        s.clear();

        fn first_letter(s: &str) -> &str {
            &s[..1]
        }
    }
}


fn compound_types_tuples() {
    {
        let t: (u8, i16) = (0, -1); // tuples can have different types, unlike arrays
        // indexing is done like this
        assert_eq!(t.1, -1);

        // we can have tuples in tuples
        let t1: (u8, (u8,i16)) = (2, (1, 30));
        // here you can clearly see that String doesn't have to be called &String, because
        // 1 of it's type, but secondly the type itself is already a ptr to heap mem.
        let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
        // indexing the inner tuple
        println!("{:?}", (t1.1).0); // we need the '()' so that the compiler knows not float
        // we can also do it by destructuring the tuple 
        let (_, (_, inner)) = t1;
        println!("{:?}", inner);
        // most small tupes can be printed out, but long tuples can't (over 12 elements)
        let long_tuple = (1,2,3,4,5,6,7,8,9,10,11,12); //this is the limit
        println!("{:?}",long_tuple);
        println!("Success!");
    }

    {
        // More destructuring stuff
        let tup = (1, 6.4, "hello");

        let (x, z, y) = tup;

        println!("{} {} {}", x, y, z);

        // tuples can be used as function args and return vals
        let (x,y) = sum_multiply((2,5));

        fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
            ((nums.0 + nums.1),(nums.0 * nums.1))
            // nums.0 + nums.1, nums.0 * nums.1 is also valid
        }
        println!("{:?}", (x,y)); 
    }
}

// like classes in java/oop
// With normal Structs like User { active: true, username: String::from("blah");, etc.}, 
// we have to use key-value initialisation, but with Tuple Structs we can also just do positional
// initialisation, i.e User(True, String::from("blah"), ...), however the trade-off is that the 
// fields will not have any names, only indexes, i.e with normal structs you can do:
// User.active to access the field, but in tupe structs, you will have to do User.0
// which increases the need for documentation, and overall more confusing
fn compound_types_structs() {
    struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
    }
    
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            // Field init Shorthand, because the params and struct field names
            // are exactly the same, we can use the field init shorthand to change
            // username: username, to
            username,
            // And
            // email: email, to
            email,
            sign_in_count: 1,
        }
    }
    
    {
        let user: User = User {
            active: true,
            username: String::from("pacs"),
            email: String::from("pacs@yahoo.com"),
            sign_in_count: 999,
        };

        println!("{}", user.email);
    }

    { 
        let mut user1: User = User {
            active: true,
            username: String::from("abc"),
            email: String::from("abc@gmail.com"),
            sign_in_count: 3,
        };
        
        // updating field values
        user1.email = String::from("anotheremail@email.com");

        println!("{}", user1.email);

        // Create instances with struct update syntax
        // Naive method
        let user2 = User {
            active: user1.active,
            username: user1.username.clone(), //because we don't want to break user1 (yet)
            email: String::from("rat@gmail.com"),
            sign_in_count: user1.sign_in_count,
        };

        // Using struct update syntax
        let user2 = User {
            email: String::from("bigrat@gmail.com"),
            ..user1
        };
        
        // the .. syntax specifies the remaining fields not explicitly set should
        // have the same values as the fields in the given instance (user1)
        // the ..<instance> has to come last, but the order of the fields being
        // explicitly mentioned doesn't matter, as they are just key-value pairs
        // and whatever fields are missing will be filled out
        
        // There is a lot of magic behind the ..<instance> syntax, but it uses '='
        // like an assignment; this is because it moves the data, i.e we can no longer
        // use user1, because it is a partial move, i.e the email field of user1
        // is still available to be called, obviously all the stack alloc. values like
        // active, sign_in_count are still available, as they are just copied
        // but the String type (because they are mem ptrs, and there can only be 1 ptr
        // to the same mem addr.), so we can't call user1.username as that field's ownership
        // is now transferred to user2 
        println!("{}", user2.email);
    }

    {
        // tuple structs
        struct Colour(i32, i32, i32);
        struct Point(i32, i32, i32);

        let origin = Point(0, 0, 0);
        let p1 = Point(0, 100, 0);
        let p2 = Point(100, 0, 0);
        let p3 = Point(0, 100, 0);
        let p3 = Point(0, 0, 100);
        let p4 = Point(100, 100, 100);
        let black = Colour(0, 0, 0);
        let red = Colour(255, 0, 0);
        let white = Colour(255, 255, 255);
        let blue = Colour(0, 0, 255);
        let green = Colour(0, 255, 0);
        paint(black, "HELLO!!");
        paint(red, "HELLO!!");
        paint(white, "HELLO!!");
        paint(blue, "HELLO!!");
        paint(green, "HELLO!!");

        draw(origin);
        draw(p1);
        draw(p2);
        draw(p3);
        draw(p4);
        // now even though Colour and Point both have the same params,
        // you cannot pass them to a fn requiring another
        fn draw(point: Point){
            let x: i32 = point.0 / 50;
            let y: i32 = point.1 / 50;
            let z: i32 = point.2 / 50;
            let painted_x = paint(Colour(0,255,0), "X"); 
            let painted_y = paint(Colour(255,0,0), "Y"); 
            let painted_z = paint(Colour(0,0,255), "Z"); 
            let idx = x as usize;
            let idy = y as usize;
            let idz = z as usize;
            
            let max_steps: usize = 5;

            let origin_col: usize = (max_steps + 1) * 2; // *2 for size for z axis
            let origin_row: usize = (max_steps + 1);

            let total_rows = (origin_row * 2);
            let total_cols = (origin_col * 2); // making extra sure we have space for z
            
            // 2D grid 
            let mut grid: Vec<Vec<String>> = vec![vec![" ".to_string(); total_cols]; total_rows];

            // Y axis
            grid[0][origin_col] = paint(Colour(255,0,0), "y");
            for i in 1..=max_steps {
                let row = origin_row-i;
                grid[row][origin_col] = if i == idy {
                                            painted_y.clone()
                                        } else {
                                            "*".to_string()
                                        };
            }
            // X axis
            grid[origin_row][total_cols - 1] = paint(Colour(0,255,0), "x");
            for i in 1..=max_steps {
                let col = origin_col + i*2;
                grid[origin_row][col] = if i == idx {
                                            painted_x.clone()
                                        } else {
                                            "*".to_string()
                                        };
            }
            // Z Axis
            grid [total_rows - 1][0+max_steps+1] = paint(Colour(0, 0, 255), "z");
            for i in 1..=max_steps {
                let row = origin_row + i;
                let col = origin_col - i;
                grid[row][col] = if i == idz {
                                            painted_z.clone()
                                        } else {
                                            "*".to_string()
                                        };
            }
            
            // origin
            grid[origin_row][origin_col] = "o".to_string();

            for row in &grid {
                println!("{}", row.join("").trim_end());
            }
        }
        fn paint(colour: Colour, message: &str) -> String {
                format!("\x1b[38;2;{};{};{}m{}\x1b[0m", colour.0, colour.1,colour.2,message)
        }

        // I got distracted, but the point of this was that you cannot pass Colour to draw()
        // despite them having the same fields types, and same no. of fields
    }

    {
        // Unit-like Structs, useful when you want to implement a trait
        // on some type but you don't have any data that you want to store in the
        // type itself. traits TBD
        struct AlwaysEqual;

        let subject = AlwaysEqual;

        println!("{}",type_of(&subject));
    }

    // Why didn't we use &str instead of String in the structs?
    // the main reason why we didn't do that is because we haven't done
    // lifetime yet (TBD)
    // i.e the compiler will complain if we pass a &str without a lifetime
    // specifier, eg:
    // struct Staff {
    //      active:bool,
    //      name:&str,
    //      email:&str,
    //      salary:i32,
    //  }
    //  let staff1: Staff = Staff {
    //      active: true,
    //      name: "someone",
    //      email: "iusedtoknow@gmail.com",
    //      salary: 99999,
    //  };
    //  if we ran this, the compiler will complain the name and email fields
    //  are missing lifetime specifiers, TBD

    
    // We can use structs to improve readability by a lot
    {
        #[derive(Debug)]
        struct Rectangle {
            length: i32,
            height: i32,
        }
        let shape: Rectangle = Rectangle{
            length: dbg!(15),
            height: 25,
        };
        fn size(rectangle: &Rectangle) -> i32 {
            rectangle.length * rectangle.height
        }

        println!("area of rectangle: {}", size(&shape));
        dbg!(&shape);
    }

    // Another implementation
    {
        struct Rectangle(i32, i32);
        fn area(shape: Rectangle) -> i32 {
            shape.0 * shape.1
        }
        fn area2(shape: (i32,i32)) -> i32 {
            shape.0 * shape.1
        }
        let shape1: Rectangle = Rectangle(100, 200);
        let shape2: (i32, i32) = (100, 200);
        println!("area:{}",area(shape1));
        println!("area:{}",area2(shape2));
    }
    
    // Methods, similar to fn but defined within the context of struct/enu/traitobj
    // TBD, the first param is always self, which represents the instance of the struct
    // the method if being called on.
    {
        #[derive(Debug)]
        struct Rectangle {
            length: i32,
            height: i32,
        }

        impl Rectangle {
            // &self is short for self: &Self
            // here we use &self, because we are immutably borrowing it
            // but if we wanted to mutably borrow it, we'd use just '&mut self'
            // it's rare for that just takes ownership by just using 'self'
            // it's usually used when the method transforms self into something else
            // and you want to prevent the caller from using the original instance after
            // the transformation. ** <- for example if we had a pokemon
            // and it had an evolve method, that take itself and change it to another
            // pokemon, we'd probably just use 'self'
            fn area(&self) -> i32 {
                self.length * self.height
            }
            // Like java, we can also create getters and setters and then make the fields
            // private. Right now, shape.length will return the same thing as
            // shape.getLength(), but again, once we make the fields private,
            // we can create proper getters and setters
            fn getLength(&self) -> i32 {
                self.length
            }
            // C/C++ . and -> operators, '.' is for when you're calling a method
            // directly on the object, -> is when you're calling a method through
            // a pointer on the object.
            // Rust doesn't have a ->, instead rust has a feature called
            // automatic referencing and dereferencing
            // eg: 
            // object.something() <-> &object.something() <-> &mut object.something()
            // it matches the signature of the method, this makes borrowing implicit
            // for method receivers!!
            fn can_hold(&self, rectangle2: &Rectangle) -> bool {
                self.length > rectangle2.length && self.height > rectangle2.height
            }

            fn setLength(&mut self, length: i32) {
                self.length = length
            }

            // Associated Functions
            // All functions defined within an impl block are called associated fncs
            // because they are associated with the type named after the impl

            fn square(size: i32) -> Self {
                Self {
                    length: size,
                    height: size,
                }
            }

            // the way we called square is like: Rectange::square,
            // just like String::from !!, because it's the same type, it's
            // an associated function/method that doesn't take &self
            // and these types are often used as constructors
            // which will return a new instance of the struct
            // the '::' are used by both associated functions and 
            // namespaces created by modules
        }

        // Each struct is allowed to have multiple impl blocks,
        // here we fit every method we wanted in 1, but we could have split them
        // into multiple. In fact there's a case in which multiple impl blocks
        // are useful (TBD) when we discuss generic types and traits!

        let shape: Rectangle = Rectangle{
            length: 15,
            height: 25,
        };
        
        let mut shape2: Rectangle = Rectangle{
            length: 50,
            height: 26,
        };
        
        let shape3: Rectangle = Rectangle::square(30);

        println!("{:?}", shape3.getLength());

        println!("area of rectangle: {}", shape.area());
        println!("shape 1 can fit shape 2: {}", shape.can_hold(&shape2));
        println!("shape 2 can fit shape 1: {}", shape2.can_hold(&shape));

        shape2.setLength(100);
        println!("{:?}", shape2.getLength());
    }


}

fn compound_types_enums() {
    // Structs let you combine multiple different fields, enums gives you a way of 
    // saying those fields can only have values from a set of values
    // and you can only have one of these values for a var, i.e
    // if you had:
    // enum Light {
    //      Red,
    //      Green,
    //      Yellow,
    //  }
    //  .
    //  You can't make 1 variable both Light::Red and Light::Yellow (unless write some code for that)
    //  Also, yes, that's how you access the values within the enum, again use the the double :
    {
        enum IpAddrKind {
            V4(u8, u8, u8, u8),
            V6(String),
        } // we could've also implemented a struct IpAddr, with the enum IpAddrKind,
          // however representing the same concept just using enum is more concise
          // in fact, the std lib does it like this for ipaddr, and instead the fields
          // inside are the ones that are actually structs!!
        fn route(ip_kind: IpAddrKind) {}
        let four = IpAddrKind::V4(1,1,1,1);
        let six = IpAddrKind::V6(String::from("::1"));

    }

    { // how it's implemented in the std lib net for ipaddr
        struct Ipv4Addr {
            // --snip--
        }
        
        struct Ipv6Addr {
            // --snip--
        }
        
        enum IpAddr {
            V4(Ipv4Addr),
            V6(Ipv6Addr),
        }
    }

    // You can also make enums of enums
    // In fact Option<T> is literally an enum like this:
    // enum Option<T> {
    //      Some(T),
    //      None,
    //  }
    //  Option basically means either it'll have this value or None
    //  And Some is just 1 of the states, it's a value, not type
    //  This exists to deal with null values/references!!
    {
        #[derive(Debug)]
        enum Type {
            Fire,
            Water,
            Air,
            Earth,
        }
        // you can have multiple different fields for each as well!
        // You don't necessarily have to have the same type in each
        #[derive(Debug)]
        enum Move {
            Punch(Option<Type>),
            Kick(Option<Type>),
            Absorb(Option<Type>),
            Run,
            Talk(String),
        } 

        // it's better in cases to use an enum instead of different structs
        // because each has it's own type, we couldn't as easily
        // define a function to take any of these kinds of Moves
        // as could with the Move enum. which just a single type
        // Another similarity b/w enums and structs is that you can 
        // also define methods using impl!!
        
        impl Move {
            fn call(&self) {
                dbg!(&self);
            }
        }

        let m: Move = Move::Punch(Some(Type::Fire));
        m.call();

        let some_num = Some(5); // it can infer it's Option<i32>
        let some_char = Some('e'); // it can infer it's Option<char>
        let absent_num: Option<i32> = None; // rust needs to be told the type, as it can't infer
                                            // by just looking at the None value, Here we tell
                                            // rust we mean for absent_num to be of type
                                            // Option<i32>
        // So what is the point of Option and Some, when we can have a None value?
        // it's because Option<T> and T are different types, the compiler won't let us use
        // Option<T> value as if we were using a valid value. 
        // i.e we can't add 8_i32 to some_num
        // In other words, you have to convert an Option<T> to a T before you can perform
        // T operations with it. And this helps us catch biggest issue with null:
        // assuming that something isn't null, when it is. And the match expression is
        // a control flow construct that does just this when used with enums. It handles
        // the cases when you do have T, and also when you None, and many others!!

    }

    {
        // coin sorter!!
        #[derive(Debug)] 
        enum UsState{
            NJ,
            NY,
            DC,
            HI,
        }

        #[derive(Debug)] 
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }
        
        fn denomination(coin: &Coin) -> u8 {
            match coin {
                Coin::Penny => {
                    println!("Lucky Penny!");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    // You cannot format like this unless you use Debug
                    println!("state quarter from {state:?}!");
                    25
                }
            }
        }

        let c: Coin = Coin::Quarter(UsState::DC);
        let val: u8 = denomination(&c);
        println!("{:?}",c)
    }

    {
        // using match with Option<T>, to make handling None cases much easier!


        fn plus_one(x: Option<i32>) -> Option<i32> {
           match x {
               None => None,
               Some(i) => Some(i + 1),
           }
        }
        // matches are exhaustive, i.e will check for every value possible
        // for enums this is fine, but for Option<i32>, or similar, 
        // you have to use the catch-all pattern, 

        let five: Option<i32> = Some(5);

        let six = plus_one(five);

        let none: Option<i32> = plus_one(None); // here we have to specify the type,
                                                // as the compiler can't infer

        println!("{} {}", five.unwrap(), six.unwrap()); // five wasn't consumed as it was a
                                                        // simple i32 type which could be copied
    }


    {
        #[derive(Debug)]
        struct Player {
            hats: i32,
            position: i32,
        }
        
        impl Player {
            fn roll(&mut self){
                let mut num = rand::thread_rng().gen_range(0..=6);
                match num {
                    3 => self.add_fancy_hat(),
                    5 => self.remove_fancy_hat(),
                    6 => self.step(),
                    _ => self.roll(),
                }
                println!("{:?}", num);
            }

            fn add_fancy_hat(&mut self) {
                self.hats += 1;
            }
            fn remove_fancy_hat(&mut self) {
                self.hats -= 1;
            }
            fn step(&mut self) {
                self.position += 1;
            }
        }

        let mut p: Player = Player {
            hats: 0,
            position: 0,
        };

        p.roll();
        p.roll(); 
        println!("{:?}",p);
    }

    // Consier Control flow with if let and let else, a less verbose way to handle values
    // that match one pattern while ignoring the rest.
    // Eg:
    // let config_max = Some(3u8);
    // match config_max {
    //      Some(max) => println!("max = {max}"),
    //      _ => (),
    //  
    //  }
    //  we aren't doing anything with the None value,
    //  instead we could write this in a shorter way using if let:
    //  let config_max = Some(3u8);
    //  if let Some(max) = config_max {
    //      println!("max = {max}");
    //  }
    //  the syntax if let takes a pattern and expr separated by an equals,

    {
        // let's say here we get a value, but we want to make sure we aren't working w/
        // a null type, we can do this, like this instead of using the match
        // which is longer to write out, what we lose in having an exhaustive list
        // is conciseness
        let config_max = Some(100u8);
        if let Some(max) = config_max {
            println!("max = {max}");
        };


    }

    {
        // coin sorter!!
        #[derive(Debug)] 
        enum UsState {
            NJ,
            NY,
            DC,
            HI,
        }

        impl UsState {
            fn existed_in(&self, year: u16) -> bool {
                match self {
                    UsState::NJ => year >= 1776,
                    UsState::NY => year >= 1788,
                    UsState::DC => year >= 1889,
                    UsState::HI => year >= 1959,
                    _ => false,
                }
            }
        }


        #[derive(Debug)] 
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }
        
        fn denomination(coin: &Coin) -> u8 {
            match coin {
                Coin::Penny => {
                    println!("Lucky Penny!");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    // You cannot format like this unless you use Debug
                    println!("state quarter from {state:?}!");
                    25
                }
            }
        }
        
        let mut count = 0;
        fn count_coins_1(coin: &Coin, count: &mut i32){
            match coin {
                Coin::Quarter(state) => println!("{state:?}"),
                _ => *count += 1,
            }
        } 
        fn count_coins_2(coin: &Coin, count: &mut i32){
            // using if let
            if let Coin::Quarter(state) = coin {
                println!("{state:?}");
            } else {
                *count += 1;
            }
        }

        let c1: Coin = Coin::Quarter(UsState::NJ);
        let c2: Coin = Coin::Quarter(UsState::NY);
        let c3: Coin = Coin::Penny;
        count_coins_1(&c1, &mut count);
        count_coins_2(&c2, &mut count);
        count_coins_2(&c3, &mut count); // only non-quarter
        println!("{:?}", count); 
        
        fn describe_quarter(coin: &Coin) -> Option<String> {
            if let Coin::Quarter(ref state) = *coin { 
                // the state has to be referenced
                // it is part of an enum struct that has multiple fields
                // and doesn't implement copy, so we have to borrow
                // if we had passed coin not as a reference but as to take
                // ownership, we wouldn't have to ref state, instead
                // we could've directly used it, as the ownership would've
                // been transferred
                if state.existed_in(1900) {
                    Some(format!("{state:?} is pretty old (for the US)"))
                } else {
                    Some(format!("{state:?} is relatively new!"))
                }
            } else {
                None
            }
        }
        // This works but it has pushed the work into the body of if let statement
        // if the work to be done was more complicated, it would be hard to follow exactly
        // how the top-level branches relate. 
        
        fn describe_quarter2(coin: &Coin) -> Option<String> {
            // here state is state if the 'if let' conditional is true
            // otherwise it's None
            let state = if let Coin::Quarter(ref state) = *coin { 
                state
            } else {
                return None; //quiting the function early, we finally use return!!
            };
            // if the above if let 
            if state.existed_in(1900) {
                Some(format!("{state:?} is pretty old (for the US)"))
            } else {
                Some(format!("{state:?} is relatively new!"))
            }
        }
        // This also works but is also confusing in it's own way, one branch of the 
        // if let produces a value and another returns from the function entirely
        // To make this common pattern nicer to express, rust has 'let..else'
        // syntax, which is similar to 'if let' in the sense it takes a pattern
        // on the left side and an expression on the right, but it doesn't have an
        // 'if' branch, only an else branch. If the pattern matches, it will bind the
        // value from the pattern to the OUTER SCOPE, i.e you can use it outside the let.. else
        // block
        // ,otherwise it'll flow to the else arm and then exit the function with None, but if
        // we didn't use return, we could instead return to the outscope a different value
        // for example we changed None to material, or something else, and then based on the 
        // material of the coin, describe it based on that insteead or something else,
        // but we don't necessarily have to quit immediately, and whatever the let..else
        // pattern evaluates to
        
        fn describe_quarter3(coin: &Coin) -> Option<String> {
            // Here we don't have to say let state = ...
            // because with the 
            let Coin::Quarter(ref state) = *coin else{ // this is in essence the same as let state ...
                                                  // because we are destructuring and getting state
                                                  // out of coin
                return None; //quiting the function early
            };
            // if the above if let 
            if state.existed_in(1900) {
                Some(format!("{state:?} is pretty old (for the US)"))
            } else {
                Some(format!("{state:?} is relatively new!"))
            }
        }
        
        let desc: Option<String> = describe_quarter(&c1);

        if let x = desc {
            println!("{:?}",x);
        } else {
            println!("not a quarter!");
        };
    }
}

        

// using some generics so that I can return the mem. addr. of any
// type!! :)
fn get_addr<T>(var: &T) -> String {
    format!("{:p}", var) // basically similar to the println! macro
                         // however it will not print it out to 
                         // stdio? what does rust print out to? TBD
}
