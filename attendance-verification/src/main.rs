#[derive(Debug)]
enum Fruit {
    Apple(String),
    Banana(String),
    Tomato(String),
}

struct Inventory {
    fruit: Vec<Fruit>,
}

impl Inventory {
    fn available_fruits(&self) {
        for f in &self.fruit{
            print!("{:?}: ",f);
            Self::tell_me_joke(f);
        }
    }

    fn tell_me_joke(fruit: &Fruit) {
        match fruit{
            Fruit::Apple(msg) => println!("{msg}"),
            Fruit::Banana(msg) => println!("{msg}"),
            Fruit::Tomato(msg) => println!("{msg}"),
        }
    }
}

fn main() {
    let a = "My granddaughter is the apple of my eye.".to_string();
    let b = "Eat a banana when you are sore, and get your mind ready for more.".to_string();
    let t = "Tomato on your plate, keeps your cardio great.".to_string();
    let fruits = vec![Fruit::Banana(b), Fruit::Apple(a), Fruit::Tomato(t)];
    let grocery_store = Inventory { fruit: fruits };
    grocery_store.available_fruits();
}
