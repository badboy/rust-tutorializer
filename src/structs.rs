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

fn build_store() -> Store {
    let mut store = Store::new(format!("Rustmart"));
    store.add_item(Item { name: "chocolate", price: 5.0 });
    store.add_item(Item { name: "socks", price: 23.0 });
    store.add_item(Item { name: "plush Mozilla dinosaur", price: 13.0 });
    store
}

#[test]
fn total_price() {
    let store = build_store();
    let list = vec!["chocolate", "plush Mozilla dinosaur"];
    assert_eq!(store.total_price(&list), 18.0);
}

