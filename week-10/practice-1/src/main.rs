fn main() {
    let v = vec![101, 250, 330 , 400];
    let v2 = &v; //this lets you access v without to v2 without v losing itsownership to the variable by referencing it.
    println!("{:?}", v);            
}
