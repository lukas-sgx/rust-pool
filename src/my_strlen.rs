pub fn my_strlen(input: &str) -> usize {
    let mut  c = 0;
    
    for _n in input.chars() {
        c += 1;
    }
    c
}