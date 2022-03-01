// use std::mem::size_of;
use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_char;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    // let a: usize = 42;
    // let b: &[u8; 10] = &B;
    // let c: Box<[u8]> = Box::new(C);

    // println!("a (符号のない整数):");
    // println!(" 場所: {:p}", &a);
    // println!(" サイズ: {:?}　バイト", size_of::<usize>());
    // println!(" 値: {:?}", a);
    // println!();

    // println!("b (Bへの参照):");
    // println!(" 場所: {:p}", &b);
    // println!(" サイズ: {:?}　バイト", size_of::<&[u8; 10]>());
    // println!(" 値: {:?}", b);
    // println!();

    // println!("c (Cを入れたボックス):");
    // println!(" 場所: {:p}", &c);
    // println!(" サイズ: {:?}　バイト", size_of::<Box<[u8]>>());
    // println!(" 値: {:?}", c);
    // println!();

    // println!("B (10バイトの配列):");
    // println!(" 場所: {:p}", &B);
    // println!(" サイズ: {:?}　バイト", size_of::<[u8; 10]>());
    // println!(" 値: {:?}", B);
    // println!();

    // println!("C (11バイトの配列):");
    // println!(" 場所: {:p}", &C);
    // println!(" サイズ: {:?}　バイト", size_of::<[u8; 11]>());
    // println!(" 値: {:?}", C);
    // println!();

    let a = 42;
    let b: String;
    let c: Cow<str>;

    unsafe {
        let b_ptr = &B as *const u8 as *mut u8;
        b = String::from_raw_parts(b_ptr, 10, 10);
        let c_ptr = &C as *const u8 as *const c_char;
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }

    println!("a: {}, b: {}, c: {}", a, b, c);
}
