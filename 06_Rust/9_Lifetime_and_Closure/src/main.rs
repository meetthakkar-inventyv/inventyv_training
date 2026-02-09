use std::{collections::HashMap, fmt::Display};
#[derive(Debug,Clone)]
struct Inventory <'a, T>
where T: DisplayItem + Clone + 'a
{
    items: HashMap<String,&'a T>
}

#[derive(Debug,Clone)]
#[allow(dead_code)]
struct Product{
    id: u32,
    name: String
}


trait DisplayItem{
    fn display(&self) -> String;
}

///implementing display item for string
impl DisplayItem for String{
    fn display(&self) -> String{
        format!("{}",self)
    }
}

// #[derive(Debug)]
enum InventoryError {
    DuplicateId,
    ItemNotFound,
    InvalidId
}

//Implementing Display for InventoryError to print custom error message
impl Display for InventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            InventoryError::DuplicateId => write!(f,"Duplicate Id"),
            InventoryError::ItemNotFound => write!(f,"Item Not Found"),
            InventoryError::InvalidId => write!(f,"Invalid Id"),
        }
    }
}

///implementing display item for Product
impl DisplayItem for Product{
    fn display(&self) -> String{
        format!("{:#?}",self)
    }
}

///Implementing Inventory
impl <'a, T> Inventory<'a, T>
    where T: DisplayItem + Clone + 'a
{   
    fn add_item(&mut self, id: String, item: &'a T) -> Result<(), InventoryError> {
        
        if id.is_empty() {
            return Err(InventoryError::InvalidId);
        }
        
        if self.items.contains_key(&id) {
            return Err(InventoryError::DuplicateId);
        }
        self.items.insert(id, item);
        Ok(())
    }


    
    fn find_by_id(&self, id: String) -> Result<&'a T, InventoryError> {
        match self.items.get(&id) {
            Some(item) => Ok(item.clone()),
            None => Err(InventoryError::ItemNotFound),
        }
    }
    
}



fn main() {

    let display_all_string = |inv: &Inventory<String>| -> String {
        let mut output = String::new();
        for (key, value) in &inv.items {
            output.push_str(&format!(
                "Item Key: {}, Item Value: {}\n",
                key,
                value.display()
            ));
        }
        output
    };

    let display_all_product = |inv: &Inventory<Product>| -> String {
        let mut output = String::new();
        for (key, value) in &inv.items {
            output.push_str(&format!(
                "Item Key: {}, Item Value: {}\n",
                key,
                value.display()
            ));
        }
        output
    };
    
    println!("//--------------------------------------------- T as String -----------------------------------------------------");
    let mut i1: Inventory<String> = Inventory{
        items: HashMap::new()
    };
    let temp = "Item 1".to_string();
    let res1 = i1.add_item("1".to_string(), &temp);
    match res1 {
        Ok(()) => println!("Item added successfully"),
        Err(e) => println!("Error : {}",e),
    }
    let temp = "Item 2".to_string();
    let res2 = i1.add_item("2".to_string(), &temp);
    match res2 {
        Ok(()) => println!("Item added successfully"),
        Err(e) => println!("Error : {}",e),
    }
    let temp = "Item 1".to_string();
    let res3 = i1.add_item("".to_string(), &temp);
    match res3 {
        Ok(()) => println!("Item added successfully"),
         Err(e) => println!("Error : {}",e),
    }
    let temp = "Item 1".to_string();
    let res4 = i1.add_item("1".to_string(), &temp);
    match res4 {
        Ok(()) => println!("Item added successfully"),
         Err(e) => println!("Error : {}",e),
    }

    let f1 = i1.find_by_id("2".to_string());
    match f1{
        Ok(item) => println!("Item found : {}",item),
        Err(e) => println!("Error : {}",e),
    }

    let f2 = i1.find_by_id("3".to_string());
    match f2{
        Ok(item) => println!("Item found : {}",item),
        Err(e) => println!("Error : {}",e),
    }
    // println!("Item found : {}",f1.unwrap());
    println!("All Items: \n{}",display_all_string(&i1));



    println!("//--------------------------------------------- T as Product -----------------------------------------------------");
     let mut i2: Inventory<Product> = Inventory{
        items: HashMap::new()
    }; 
    let  p1 = Product{id: 1, name: "Item1".to_string()};
    let  p2 = Product{id: 2, name: "Item2".to_string()};
    let res_p1 = i2.add_item("1".to_string(), &p1);
    match res_p1{
        Ok(()) => println!("Item added successfully"),
        Err(e) => println!("Error : {}",e),
    }

    let res_p2 = i2.add_item("2".to_string(), &p2);
    match res_p2{
        Ok(()) => println!("Item added successfully"),
        Err(e) => println!("Error : {}",e),
    }
    let binding = Product{id: 1, name: "Item1".to_string()};
    let res_p3 = i2.add_item("".to_string(), &binding);
    match res_p3{
        Ok(()) => println!("Item added successfully"),
        Err(e) => println!("Error : {}",e),
    }
    let binding = Product{id: 1, name: "Item1".to_string()};
    let res_p4 = i2.add_item("1".to_string(), &binding);
    match res_p4{
        Ok(()) => println!("Item added successfully"),
        Err(e) => println!("Error : {}",e),
    }

    let f_p1 = i2.find_by_id("2".to_string());
    match f_p1{
        Ok(item) => println!("Item found : {:#?}",item),
        Err(e) => println!("Error : {}",e),
    }

    let f_p2 = i2.find_by_id("3".to_string());
    match f_p2{
        Ok(item) => println!("Item found : {:#?}",item),
        Err(e) => println!("Error : {}",e),
    }
    println!("All Items: \n{}",display_all_product(&i2));

}
