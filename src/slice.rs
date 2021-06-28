//Slice lets you reference a contiguous sequence of elements in a collection rather than the whole collection


// pub fn slice_to(s: &String) {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate(){
//         if item == b' '{
//             return i;
//         }
//     }
//
// }

// fn shock() {
//     let mut s = String::from("hello world");
//     let word = first_word(&s); // word will get the value 5
//     s.clear(); // this empties the String, making it equal to ""
// // word still has the value 5 here, but there's no more string that
// // we could meaningfully use the value 5 with. word is now totally
// // invalid!
// }