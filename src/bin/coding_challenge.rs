#[derive(Debug)]
struct SupermaketItem {
    name: String,
    price: f64,
}

#[derive(Debug)]
struct ShoppingCart {
    items: Vec<SupermaketItem>,
}

impl ShoppingCart {
    fn traverse_items<F>(&mut self, mut operation: F)
    where
        F: Fn(&mut SupermaketItem),
    {
        let final_index = self.items.len() - 1;
        let mut current_index = 0;
        while current_index <= final_index {
            let current_item = &mut self.items[current_index];
            operation(current_item);
            current_index += 1;  
        } 
    }

    fn checkout<F>(self, operation: F)
    where
        F: FnOnce(Self),
    {
        operation(self);
    }
}
    

fn main() {
    let mut shopping_cart = ShoppingCart {
        items: vec![
            SupermaketItem {
                name: "APPLE".to_string(),
                price: 3.99,
            },
            SupermaketItem {
                name: "BANANA".to_string(),
                price: 2.99,
            },
        ],
    };

    shopping_cart.traverse_items(|item| {
        item.price *= 0.85;
    });

    shopping_cart.traverse_items(|item|{
        item.name = item.name.to_lowercase();
    });

    let mut total_price = 0.0;

    shopping_cart.checkout(|mut cart| {
        println!("{cart:?}");
        cart.traverse_items(|item| {
            total_price += item.price;
        });
    });

    println!("${total_price:.2}");
}