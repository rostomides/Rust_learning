
fn main() {
    for b in "Bonjour".bytes() {
        println!("{}", b);
    }

    println!("{}", "oooogggle".find("og").unwrap());
}


// fn main() {
//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }

//     let mut vect = vec![
//         SpreadsheetCell::Int(3),        
//         SpreadsheetCell::Float(10.12),
//         SpreadsheetCell::Text(String::from("blue")),
//     ];

    

    

//     // println!("Second element is : {}", &vect.printelm);

// }



// fn main() {

//     let mut v = vec![1, 2, 3, 4, 5];    

//     v.push(6);
//     v.pop();

//     let first = &v[1];
    

//     println!("The first element is: {}", first);




// }




// fn main(){

//     struct Color(i32,i32,i32);

//     let c1 = Color(1,4,6);

//     println!("{} {} {}", c1.0,c1.1, c1.2);

//     println!("{} {} {}", c1.0,c1.1, c1.2);

// }






// fn main(){

//     let s = String::from("hello");

//    let slice = slice(&s);

//     println!("{}", slice);

    
// }

// fn slice(s:&String) -> (&str, usize) {
//     // let slice = &s[0..2];
//     return (&s[..2], &s[..2].to_string().len());
// }

