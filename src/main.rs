
fn main() {
   let a = [1, 2, 3, 5];
   let slice = &a[1..3];
   assert_eq!(slice, &[2, 3]);
}
