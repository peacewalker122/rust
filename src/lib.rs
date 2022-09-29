pub struct Point{
    pub(crate) x: i64,
    pub(crate) y: i64,
}
pub struct Rectangle{
    pub(crate) p1: Point,
    pub(crate) p2: Point,
}
pub struct Encrypt{
    pub(crate) p1: Point,
}

pub fn sumodd_number(up_to:u32) -> u32{
    let mut acc = 0;
        for i in 0..up_to+1 {
            let add: u32 = match i%2 == 1 {
                true => i,
                false => continue,
            };
            acc += add;
        }
        acc
    }
    
pub fn lcm(num1: i64,num2: i64)->i64{
        let gcd = gcd(num1, num2);
        let multiply = (num1 * num2).abs();
        if gcd == 0{
            println!("`lcm fn`: Can't accept zero value");
            return 0
        }
        multiply/gcd
        
    }
    
pub fn gcd(mut num1: i64,mut num2: i64)->i64{
       let mut r: i64;
        while num1 % num2 > 0 {
            r = num1 % num2;
            num1 = num2;
            num2 = r;
        };
        if num2 == 1{
            return 0
        }
        return num2;
    }
    
pub fn odd_or_even(num: i32){
        if num % 2 == 0 {
         println!("{} is even",num);
         return;
        } else {
            println!("{} is odd",num);
            return;
        }
    }
    
pub fn bubblesort<T: Ord>(array:&mut[T]) {
        for i in 0..array.len() {
            for j in 0..array.len()-1-i{
                
                if array[j] < array[j+1]{
                    array.swap(j, j+1)
                }
            }
        }
    }
    
    
    impl Point {
        // this function below for taking the value by copying value.
        pub fn origin() -> Point{
            Point { x: 0, y: 0 }
        }
    
        pub fn new(x: i64 , y: i64)->Point{
            Point { x: x, y: y }
        }
    }
    
    impl Rectangle {
       pub fn area(&self)->i64{
            let Point { x:x1, y:y1 } = self.p1;
            let Point { x:x2, y:y2 } = self.p2;
            // abs = to ensure the return value is number that > 0.
            ((x1 - x2) * (y1 - y2)).abs()
        }
    
        pub fn perimeter(&self)->i64{
            let Point { x:x1, y:y1 } = self.p1;
            let Point { x:x2, y:y2 } = self.p2;
    
            2 * ((x1 - x2).abs() + (y1 - y2).abs())
        }
    
        pub fn translate(&mut self, x: i64, y: i64) {
            self.p1.x += x;
            self.p2.x += x;
    
            self.p1.y += y;
            self.p2.y += y;
        }
    }
    
    impl Encrypt{
        pub fn gcd(&self)->i64{
            let Point { x:mut x1, y:mut y1 } = self.p1;
    
            let mut r: i64;
            while x1 % y1 > 0 {
                r = x1 % y1;
                x1 = y1;
                y1 = r;
            };
            if y1 == 1{
                return 0
            }
            return y1.abs();
        }
    
       pub fn lcd(&self)->i64{
            let Point { x: x1, y: y1 } = self.p1;
    
            let gcd = self.gcd();
            let multiply = (x1 * y1).abs();
            if gcd == 0{
                println!("`lcm fn`: Can't accept zero value");
                return 0
            }
    
            multiply/gcd
    
        }
    }
    pub mod my_mod{
        pub fn starprint(length:i64){
            //let mut k:i64 = 0;
            for i in 0..length+1{
                for _ in 0..length-i {
                    print!(" ")
                }
                for _ in 0..i {
                    print!("* ");
                }
                println!("");
            }
        }
        pub mod nested{
            pub fn hello(s: String){
                println!("Hello, {}",s)
            }
        }
    }