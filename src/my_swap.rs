pub fn my_swap(dest: &mut i32, src: &mut i32) {
    let temp;

    temp = *dest;
    *dest = *src;
    *src = temp;
}