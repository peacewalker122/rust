mod lib;
use lib::my_mod::nested::hello as hellofunc;
use lib::SetMath;

fn main() {
    let y = 7 + 5;
    let p: String = String::from("test");

    let mut m = [2,3,7,5,99,8919,2626,4626,383773,39839783,373873,38337,3712,122,1273,1,4];
    
    println!("Hello, world {}",p);
    println!("I'm rustacean {}",y);
    println!("hexa{:x}",292297872);

    for z in m {
        lib::odd_or_even(z)
    }
    
    lib::bubblesort(&mut m);
    println!("result: {:?}",m);

    let mut rectangle = lib::Rectangle{
         p1:lib::Point::origin(), 
         p2:lib::Point::new(3, 4), 
        };

    println!("rectangle perimeter is: {}",rectangle.perimeter());
    println!("rectangle area is: {}",rectangle.area());
    rectangle.translate(40,20);
    println!("rectangle update perimeter is: {}",rectangle.perimeter());
    println!("rectangle update area is: {}",rectangle.area());

    println!("The GCD is {}",lib::gcd(800, 90));
    println!("The LCM is {}",lib::lcm(800, 90));
    println!("Sum of odd numbers up to 9 (excluding): {}", lib::sumodd_number(9));

    let encyrpt = lib::Encrypt{
        p1: lib::Point { x: 800, y: 90 },
    };

    println!("The gcd is {}",encyrpt.gcd());
    println!("The lcd is {}",encyrpt.lcd());

    let x1:[i64;10] = [2,32,2,5,66,87,88,8,34,3];
    let y1:[i64;10] = [29,32,39,4,76,67,20,8,34,3];

    let sets = lib::Sets{
        p1:lib::Set::new(x1, y1)
    };
    println!("the Total sum of the two array is: {:?}",sets.sum());
    println!("the Union of the two array is: {:?}",sets.union());
    println!("printing pyramid");
    lib::my_mod::starprint(10);

    println!("printing hellofunc");
    hellofunc("Pigeon".to_string())
}