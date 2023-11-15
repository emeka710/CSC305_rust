fn value_in_cents(coin: Coin) -> u8 {
    enum Shape {
        Circle(f64),
        Rectangle(f64, f64),
        Square(f64),
    }

    let coin = Coin::Quarter;
let circle = Shape::Circle(2.5);
let rectangle = Shape::Rectangle(3.0, 4.0);
    
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}