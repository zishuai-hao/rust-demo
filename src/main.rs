
use std::fmt::{Display, format, Formatter};
use std::hash::*;
use std::collections::hash_map::DefaultHasher;
use std::io::Read;
fn main() {


    // print_demo();
    // tuple_demo();
    // array_demo();
    // format_demo();
    // enum_demo()
    // variable_demo()
    // type_demo()
    // cycle_demo()
    // closure_demo()
    // scope_demo()
    // trait_demo()
    // ops_trait_demo()
    // thread_demo()
    io_demo()
}


fn hello_word() {
    println!("Hello, world!");
}


/// doc comment: 文档注释
fn comment() {
    // this is single line comment

    // this is document comment,compile with --doc command you can get a html
}


pub fn print_demo() {
    // string s = "ss";
    // string  s = format!("{} years", 10);
    println!("{} days", 31);
    println!("{name} is your name?", name = "the lazy dog")
}

pub fn tuple_demo() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    println!("long tuple first value: {}", long_tuple.2);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);
}

pub fn array_demo() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("all element of the array: {:?}", ys);
}

fn format_demo() {
    // #[derive(Debug)]
    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl Display for City {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
            write!(f, "{}: {:.3}°{} {:.3}°{}",
                   self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }

    for city in [City { name: "dublin", lat: 53.3, lon: -6.33 }].iter() {
        println!("{}", *city);
    }
}

fn struct_demo() {
    struct Unit;
    struct Point(f32, f32);
    struct Image {
        height: f32,
        width: f32,

    }
    let p1 = Point(1.0, 2.0);

    let p2 = Point(1.0, p1.1);

    let i1 = Image { height: 100.0, width: 200.0 };

    // 解构拷贝
    let i2 = Image { height: 200.0, ..i1 };

    // 解构
    let Image { height: h, width: w } = i1;

    println!("height: {}, width: {}", h, w);

    type I = Image;

    let i3 = I { height: 30.0, width: 20.5 };
}

fn enum_demo() {
    use List::*;
    enum List {
        // 当前元素、下一元素
        Cons(u32, Box<List>),
        Nil,
    }

    impl List {
        fn new() -> List {
            Nil
        }

        fn prepend(self, elem: u32) -> List {
            Cons(elem, Box::new(self))
        }

        fn len(&self) -> u32 {
            match *self {
                Cons(_, ref tail) => 1 + tail.len(),
                Nil => 0
            }
        }

        fn stringify(&self) -> String {
            match *self {
                Cons(head, ref tail) => {
                    format!("{},{}", head, tail.stringify())
                }
                Nil => {
                    "Nil".to_string()
                }
            }
        }
    }

    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("{}", list.stringify());
}

fn variable_demo() {
    let mut mutable = 1;

    // let immutable = 1;

    {
        let immutable = mutable;

        // immutable += 1;
    }
    mutable += 1;

    // immutable += 1;
    println!("{}", mutable);
}

fn type_demo() {
    let decimal = 65.4321_f32;

    let integer = decimal as u8;

    let character = integer as char;

    println!("Casting {} -> {} -> {}", decimal, integer, character);


    // 带后缀的字面量，其类型在初始化时已经知道了。
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 无后缀的字面量，其类型取决于如何使用它们。
    let i = 1;
    let f = 1.0;

    // `size_of_val` 返回一个变量所占的字节数
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    // from
    use std::convert::From;
    #[derive(Debug)]
    struct Number {
        value: i32,
    }
    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }
    impl std::fmt::Display for Number {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "format value: {}", self.value.to_string())
        }
    }

    // let num = Number::from(30);
    let x1: Number = 35.into();

    println!("{}", x1);
}

fn cycle_demo() {
    let result = 'outer: loop {
        println!("entered the outer loop");
        'inner: loop {
            println!("entered the inner loop");

            // break 100;

            break 'outer 200;
        }
        println!("This point will never be reached");
    };


    println!("exited the outer loop");

    println!("{:?}", assert_eq!(200, result));

    for n in 1..=105 {
        if n % 15 == 0 {
            println!("{}", n);
        }
    }
}

fn closure_demo() {
    let closure_inferred = |i| i + 1;
    let value = closure_inferred(1);

    println!("{}", value);


    let color = String::from("green");

    let print = || println!("`color`: {}", color);

    print();
}

fn generic_demo() {
    struct Single<T>(T);

    let s: Single<i32> = Single(100);

    struct S<T: Display>(T);

    // let s1 = S(vec![1]);
}

fn scope_demo() {
    struct ToDrop;
    impl Drop for ToDrop {
        fn drop(&mut self) {
            println!("ToDrop is being dropped")
        }
    }

    let x = ToDrop;
    println!("made a ToDrop!")
}

fn trait_demo() {
    trait Animal {
        fn new(name: &'static str) -> Self;
        fn name(&self) -> &'static str;
    }

    struct Sheep {
        naked: bool,
        name: &'static str,
    }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }

        // 传入可变引用
        fn shear(&mut self) {
            if self.is_naked() {
                println!("{} is already naked...", self.name())
            } else {
                println!("{} gets a haircut!", self.name());
                self.naked = true;
            }
        }
    }

    impl Animal for Sheep {
        fn new(name: &'static str) -> Self {
            Sheep { name, naked: false }
        }

        fn name(&self) -> &'static str {
            self.name
        }
    }

    let mut animal: Sheep = Animal::new("shaun");

    animal.shear();
    animal.shear()
}

fn ops_trait_demo() {
    use std::ops;
    struct Foo;
    struct Bar;

    #[derive(Debug)]
    struct FooBar;

    #[derive(Debug)]
    struct FooFoo;

    #[derive(Debug)]
    struct BarFoo;

    impl ops::Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, rhs: Bar) -> Self::Output {
            println!("> Foo.add(Bar) was called");
            FooBar
        }
    }

    impl ops::Add<Foo> for Foo {
        type Output = FooFoo;

        fn add(self, rhs: Foo) -> Self::Output {
            println!("> Foo.add(Foo) was called");
            FooFoo
        }
    }

    impl ops::Add<Foo> for Bar {
        type Output = BarFoo;

        fn add(self, rhs: Foo) -> Self::Output {
            println!("> Bar.add(Foo) was called");
            BarFoo
        }
    }

    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Foo + Foo = {:?}", Foo + Foo);
    println!("Bar + Foo = {:?}", Bar + Foo);
}

fn thread_demo() {
    use std::thread;
    static NTHREADS: i32 = 10;

    let mut children = vec![];
    for i in 0..NTHREADS {
        // 创建线程
        children.push(thread::spawn(move || {
            // println!("this is thread number {}", i);
        }))
    }

    for child in children {
        // 启动并等待结果
        let _ = child.join();
    }

    use std::sync::mpsc::{Sender, Receiver};
    use std::sync::mpsc;

    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..NTHREADS {
        let thread_tx = tx.clone();
        thread::spawn(move || {
            thread_tx.send(id).unwrap();
            println!("thread {} finished", id);
        });
    }

    let mut ids = Vec::with_capacity(NTHREADS as usize);

    for _ in 0..NTHREADS {
        ids.push(rx.recv());
    }

    println!("{:?}", ids);
}

fn io_demo(){
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    let path = Path::new("./hello1.txt");
    let display = path.display();
    println!("{}", display);

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {} :{:?}", display, why),
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => print!("{} contains:\n{}", display, s),
        Err(why) => panic!("couldn't read {}, {:?}", display, why),
    }
}