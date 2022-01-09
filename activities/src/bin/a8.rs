// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavors {
    Strawberry,
    Mango,
    Chocolate,
    Coffee,
    Oreo,
    Nutella,
}

struct Drink {
    flavor: Flavors,
    quantity: i32,
}

fn check_drink(drink: Drink) {
    let mut flavor = "";
    match drink.flavor {
        Flavors::Strawberry => flavor = "Strawberry",
        Flavors::Mango => flavor = "Mango",
        Flavors::Chocolate => flavor = "Chocolate",
        Flavors::Coffee => flavor = "Coffee",
        Flavors::Oreo => flavor = "Oreo",
        Flavors::Nutella => flavor = "Nutella",
    }
    println!("The flavor of the drink is {:?} and its quantity is {:?} ounces", flavor, drink.quantity);
}

fn main() {
    let coffee = Drink {
        flavor: Flavors::Coffee,
        quantity: 8,
    };
    let strawberry = Drink {
        flavor: Flavors::Strawberry,
        quantity: 12,
    };
    let chocolate = Drink {
        flavor: Flavors::Chocolate,
        quantity: 10,
    };
    let mango = Drink {
        flavor: Flavors::Mango,
        quantity: 12,
    };
    let nutella = Drink {
        flavor: Flavors::Nutella,
        quantity: 6,
    };
    check_drink(coffee);
    check_drink(mango);
    check_drink(nutella);
}
