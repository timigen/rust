mod iteration;
mod file_io;

fn main() {
    println!("\n\nfile io\n");
    file_io::open();
    
    println!("iteration");
    iteration::loops();
}
