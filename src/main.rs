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
    compound_types();
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


fn compound_types() {
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
}



// using some generics so that I can return the mem. addr. of any
// type!! :)
fn get_addr<T>(var: &T) -> String {
    format!("{:p}", var) // basically similar to the println! macro
                         // however it will not print it out to 
                         // stdio? what does rust print out to? TBD
}
