
pub fn structures(){
    struct Person {
        name: String,
        age: u32,
        is_student: bool,
    }

    let person1 = Person {
        name: String::from("Alice"),
        age: 25,
        is_student: true,
    };

println!("Name: {}", person1.name);
println!("Age: {}", person1.age);
println!("Is Student: {}", person1.is_student);

}