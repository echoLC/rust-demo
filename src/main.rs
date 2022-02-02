
fn main() {
   let condition = true;
   let number = if condition {
       5
   } else {
       6
   };
   println!("the number value is: {}\n", number);

   for i in 1..= 5 {
       println!("{}\n", i);
   }

   let a = [4, 3, 2, 1];
   for (index, value) in a.iter().enumerate() {
    print!("第{}个元素的值是{}\n", index, value); 
   }

   let mut index = 0;
   for _ in 0..10 {
       index = index + 1;
       print!("我执行了{}次\n", index);
   }

   for i in 1..4 {
       if i == 2 {
           continue;
       }
       println!("{}\n", i);
   }

   let mut n = 0;
   while n <= 5 {
        println!("the n value is: {}", n);
       n = n + 1;
   }
   println!("我跳出了 while 循环，the n value is: {}", n);
}
