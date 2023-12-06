fn main() {
    let x = 1;

    println!("x = {}", x);

    soma_um(x);

    println!("x = {}", x);
}

fn soma_um(x: i32){
    println!("x+1 = {}", x+1)
}