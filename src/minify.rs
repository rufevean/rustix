pub fn minify(contents : &str) -> String{
    println!("Minifying: {}", contents);
    let mut minified = String::new();
    for c in contents.chars(){
        if c.is_whitespace() {
            continue;
        }
        minified.push(c);
    }
    minified
}
