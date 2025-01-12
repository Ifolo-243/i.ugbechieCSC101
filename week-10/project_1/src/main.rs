fn main() {
    // Prices of laptops
    let hp_price = 650_000;
    let ibm_price = 755_000;
    let toshiba_price = 550_000;
    let dell_price = 850_000;

    // Quantity purchased for each brand
    let quantity = 3;

    // Calculate the total cost
    let total_cost = (hp_price * quantity)
        + (ibm_price * quantity) + (toshiba_price * quantity)
        + (dell_price * quantity);

    // Output the results
    println!("The total cost for purchasing 3 laptops from each brand is ${}", total_cost);
}



    println!("Hello, world!");
}
