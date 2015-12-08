#![allow(dead_code)]

struct Store {
    name: String,
    items: Vec<Item>,
}

#[derive(Debug)]
struct Item {
    name: &'static str,
    price: f32,
}

impl Store {
    fn new(name: String) -> Store {
        Store {
            name: name,
            items: vec![],
        }
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn price(&self, item_name: &str) -> f32 {
        for item in &self.items {
            if item.name == item_name {
                return item.price;
            }
        }

        panic!("no such item {:?}", item_name);
    }

    fn total_price(&self, shopping_list: &[&str]) -> f32 {
        // PROMPT 0.0 // TODO
        // START SOLUTION
        shopping_list.iter()
                     .map(|name| self.price(name))
                     .fold(0.0, |a, b| a + b)
        // END SOLUTION
    }
}

fn build_store() -> Vec<Store> {
    let mut stores = vec![];

    let mut store = Store::new(format!("Rustmart"));
    store.add_item(Item { name: "chocolate", price: 5.0 });
    store.add_item(Item { name: "socks", price: 23.0 });
    store.add_item(Item { name: "plush Mozilla dinosaur", price: 13.0 });
    stores.push(store);

    let mut store = Store::new(format!("Rarget"));
    store.add_item(Item { name: "chocolate", price: 2.5 });
    store.add_item(Item { name: "socks", price: 20.0 });
    store.add_item(Item { name: "plush Mozilla dinosaur", price: 20.0 });
    stores.push(store);

    stores
}

fn main() {
    let stores = build_stores();

    let shopping_list = vec!["chocolate", "plush Mozilla dinosaur"];
    let shopping_list = Arc::new(shopping_list);

    let mut futures = vec![];
    for store in stores {
        let shopping_list = shopping_list.clone();
        futures.push(thread::spawn(move || {
            let sum = store.total_price(&shopping_list);
            (store.name, sum)
        }));
    }

    let mut best = None;
    let mut best_price = INFINITY;
    for future in futures {
        let (name, sum) = future.join().unwrap();
        println!("At {}, I would spend ${}.", name, sum);
        if sum < best_price {
            best = Some(name);
        }
    }

    println!("--> Go to {}!", best.unwrap());
}

