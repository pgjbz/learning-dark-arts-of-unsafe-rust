use std::mem::MaybeUninit;

/*
SIZE is hard coded because is easy to change len of array
*/
const SIZE: usize = 10;

fn main() {
    let x = {
        //sim a macro expansion
        let mut x: [MaybeUninit<Box<u32>>; SIZE] = unsafe {
            MaybeUninit::uninit().assume_init() //assume_init is a method thats allow to "cheat with" compiler,
                                                //this method take a some of unintialized piece of memory and tell
                                                // "hey compiler, this region of memory is initialized, trust me"
                                                //and compiler trust, you believe that? lmao
        };
        for i in 0..SIZE {
            x[i] = MaybeUninit::new(Box::new(i as u32)) //create a MaybeUninit with Box value
                                                        //rust compiler assumes x[i] is initialized, drop "old value"
                                                        //and write a new value
                                                        //the alternative for this is use ptr module
                                                        //see more on rustnomicon (and doc): https://doc.rust-lang.org/nomicon/unchecked-uninit.html
        }

        unsafe {
            std::mem::transmute::<_, [Box<u32>; SIZE]>(x) //transmute MaybeUnit to final array,
                                                          //transmute take a value and tranform in another type with same size
        }
    };
    println!("{:#?}", x);
}
