// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter
// Apparently, the default behavior in rust is to delete the variable after it has been used in a function once unless specified otherwise
// To make sure that the variable isn't deleted, we have to make the functions borrow the variable instead of the default moving

struct Item {
    id: i32,
    quantity: i32,
}

fn check_quantity(item: &Item) {
    println!("The quantity of the item is {:?}", item.quantity);
}

fn check_id(item: &Item) {
    println!("The id of the item is {:?}", item.id);
}

fn main() {
    let item = Item {
        id: 00001,
        quantity: 10,
    };
    check_id(&item);
    check_quantity(&item);
}
