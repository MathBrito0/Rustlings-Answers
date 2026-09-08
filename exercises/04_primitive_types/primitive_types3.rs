fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???

    let a = "pneumoultramicroscopicossilicovulcanoconiótico";

    if a.len() >= 40 {
        println!("Wow, that's a big word!");
    } else {
        println!("Meh, I eat words like that for breakfast.");
        panic!("Word not big enough, more elements needed.");
    }
}
