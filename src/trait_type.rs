fn main() {
    let mut counter = Counter { count: 0 };
    for _ in 0..20 {
        println!("{}", counter.next());
    }

    let empty_childs = vec![];
    let childs = vec![
        Animal::new("dog", &empty_childs),
        Animal::new("cat", &empty_childs),
        Animal::new("fish", &empty_childs),
    ];
    let animal = Animal::new("parentDog", &childs);
    print_animal_info(&animal);
}

fn print_animal_info(animal: &Animal) {
    println!("Name: {}", animal.name);
    println!("Childs: {:?}", animal.childs);
}

trait Iterator {
    type Item;
    fn next(&mut self) -> Self::Item;
}

struct Counter {
    count: u32,
}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Self::Item {
        self.count += 1;
        self.count
    }
}

trait Childs<'a> {
    type Child;
    fn childs(&self) -> &'a Vec<Self::Child>;
}

#[derive(Debug)]
struct Animal<'a> {
    name: &'a str,
    childs: &'a Vec<Animal<'a>>,
}
impl<'a> Animal<'a> {
    fn new(name: &'a str, childs: &'a Vec<Animal<'a>>) -> Self {
        Self { name, childs }
    }
}
impl<'a> Childs<'a> for Animal<'a> {
    type Child = Animal<'a>;

    fn childs(&self) -> &'a Vec<Self::Child> {
        self.childs
    }
}
