use core::slice;

fn main() {

    let first_letter='A';
    let space =' ';
    let other_lang_char='Ꮔ';
    let cat_face= '😺';
    let small_number:u8=10;
    let small_number=10u8;
    let big_number=10_00_000_i32;
    let float=6.;
    let my_float:f64=5.0;
    let my_other_float:f32=7.5;
    let third_float=my_float + my_other_float as f64;



    let my_num=100;
    let my_number:u8=100; 
    println!("{}",my_num as u8 as char);
    println!("{}",my_number as char);

    println!("size of chae :{}", std::mem::size_of::<char>());
    println!("Size of string containing 'a': {}","a".len());
    println!("Size of string containing 'ß': {}","ß".len());

    let slice="Hello";
    println!("Slice is {} bytes", slice.len());
    println!("Slice is {} bytes and in chars {}", slice.len(),slice.chars().count());
    let slice2="안녕";
    println!("Slice2 is {} bytesand in chars {}", slice2.len(), slice2.chars().count());








    
}
