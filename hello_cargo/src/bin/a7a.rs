
enum Haa{
    Ha,Haaa,Haha,HHHHHHa
}

fn print_color(hh:Haa){
    match hh {
        Haa::Ha=>println!("haa"),
        Haa::Haaa=>println!("haaa"),
        Haa::Haha=>println!("haha"),
        Haa::HHHHHHa=>println!("HHHHHa"),
    }
}

fn main(){
    let hh  = Haa::Ha;
   print_color(hh)
}