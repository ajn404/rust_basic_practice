enum Flavor {
    Spark,
    Sweet,
    Fruity,
}

struct drink {
    flavor:Flavor,
    fluid_oz:f64,
}

fn print_fn(dr:drink){
    match dr.flavor{
        Flavor::Spark=>println!("spark"),
        Flavor::Fruity=>println!("fruity"),
        Flavor::Sweet=>println!("Sweet"),
    }
    println!("{:?}",dr.fluid_oz);
}

fn main(){
    let dr = drink{
        flavor:Flavor::Spark,
        fluid_oz:1.5,
    };
    let cl = dr.fluid_oz.clone();
    print_fn(dr);
    println!("{:?}",cl);

  
} 