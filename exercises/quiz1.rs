// quiz1.rs
//
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//

// Function to calculate the price of apples based on quantity
fn calculate_price_of_apples(quantity: i32) -> i32 {
    if quantity > 40 {
        quantity // If more than 40 apples, each costs 1 rustbuck
    } else {
        quantity * 2 // Otherwise, each apple costs 2 rustbucks
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
