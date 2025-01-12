fn main() {
    // problem 1: we have some_product of type Some(&str) or None
    // we also have vector of products. we want to add some_product to the vector if only it is not None

    // solution 1
    let some_product = Some("laptop");
    let mut products = vec!["cellphone", "battery", "adapter"];
    match some_product {
        Some(product) => products.push(product),
        _ => {}
    }

    // solution 2 using if let
    let some_product = Some("laptop");
    let mut products = vec!["cellphone", "battery", "adapter"];
    if let Some(p) = some_product {
        products.push(p);
    }

    // solution 3 using extend method
    let some_product = Some("laptop");
    let mut products = vec!["cellphone", "battery", "adapter"];
    // if only some_product is not None, it will be added to the vector
    products.extend(some_product);

    let none_prod: Option<&str> = None;
    // wont add anything
    products.extend(none_prod);

    println!("{:?}", products);

    // solution 3: using iterator and chain
    let some_product = Some("laptop");
    let products = vec!["cellphone", "battery", "adapter"];

    let products_itr = products.iter().chain(some_product.iter());
    for p in products_itr {
        println!("product: {:?}", p);
    }

    // solution 4: using iterator and chain then collect
    let some_product = Some("laptop");
    let products = vec!["cellphone", "battery", "adapter"];

    let chained_products = products
        .iter()
        .chain(some_product.iter())
        .collect::<Vec<&&str>>();
    println!("chained products: {:?}", chained_products);

    // ----------------------------------------------------------------------
    // problem 2: we have a list of products. we want to remove None variants

    // solution 1: using for loop and new vector
    let products = vec![Some("laptop"), Some("cellphone"), None, Some("Adapter")];
    println!("sol1: before removing Nones: {:?}", products);
    let mut products_without_none: Vec<&str> = Vec::new();
    for p in products {
        if p.is_some() {
            products_without_none.push(p.unwrap());
        }
    }
    println!("sol1: after removing Nones: {:?}", products_without_none);

    // solution 2: using combinator
    let products = vec![Some("laptop"), Some("cellphone"), None, Some("Adapter")];
    println!("sol2: before removing Nones: {:?}", products);
    let products_without_none = products
        .into_iter()
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .collect::<Vec<&str>>();
    println!("sol2: after removing Nones: {:?}", products_without_none);

    // solution 3: using flatten function
    let products = vec![Some("laptop"), Some("cellphone"), None, Some("Adapter")];
    println!("sol3: before removing Nones: {:?}", products);
    let products_without_none = products.into_iter().flatten().collect::<Vec<&str>>();
    println!("sol3: after removing Nones: {:?}", products_without_none);
}
