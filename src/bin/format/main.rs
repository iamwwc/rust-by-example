use std::mem::size_of;

fn main() {
    format_print_1_2();
    format_print_debug_1_2_1();
}

fn format_print_1_2() {
    println!("{} days", 31);
    
    println!("{0}, this is {1}. {1}, this is {0}","Alice", "Bob");

    println!("{subject} {verb} {object}", object="the lazy dog", 
                                            subject="the quick brown fox",
                                            verb="jumps over");
    println!("{} of {:b} people know binary, the other half doesn't",1,2);
    println!("{number:>width$}",number=1,width=6);
    println!("My name is {0}, {1} {0}","Bond", "James");
    #[derive(Debug)]
    struct Structure(bool);
    println!("This struct `{:?}` won't print...",Structure(false));
    println!("The sizeof Structure is {}",size_of::<Structure>());
    struct Structure_Bool(bool);
    struct Structure_Integer(i32);
    assert_eq!(1,std::mem::size_of::<Structure_Bool>());
    assert_eq!(4,std::mem::size_of::<Structure_Integer>());
}

fn format_print_debug_1_2_1() {
    struct UnPrintable(i32);
    // struct 需要 derive Debug trait 才能被println
    #[derive(Debug)]
    struct DebugPrintable(i32);
    #[derive(Debug)]
    struct Structure(i32);
    #[derive(Debug)]
    struct Deep(Structure);
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.","Slater","Chrisitan", actor="actor's");
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));
    println!("sizeof Deep(Structure) is {}", size_of::<Deep>());

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    // name 是 &str 类型是由于
    // 字符串存放在 .rodata 段，运行时并不分配到stack，所以直接引用到 .rodata 段的数据
    // 通过 readelf 配合 HexEditor 可以分析出来
    let name = "Peter";
    let age = 27;
    let peter = Person{ name, age};
    println!("{:#?}", peter);
}