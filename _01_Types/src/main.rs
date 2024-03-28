fn main() {

    let first_letter='A';
    let space =' ';
    let other_lang_char='Ꮔ';
    let cat_face= '😺';

    let my_num=100;
    let my_number:u8=100; 
    println!("{}",my_num as u8 as char);
    println!("{}",my_number as char);

    println!("size of chae :{}", std::mem::size_of::<char>());
    println!("Size of string containing 'a': {}","a".len());
    println!("Size of string containing 'ß': {}","ß".len());

    let slice="Hello";
    println!("Slice is {} bytes", slice.len());
    





    
}
