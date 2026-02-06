use std::{collections::HashMap, fmt::Display};
#[derive(Debug,Clone)]
struct Inventory <T>
where T: DisplayItem + Clone{
    items: HashMap<String,T>
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
impl <T> Inventory<T>
    where T: DisplayItem + Clone
{   
    fn add_item(&mut self, id: String, item: T) -> Result<(), InventoryError> {
        
        if id.is_empty() {
            return Err(InventoryError::InvalidId);
        }
        
        if self.items.contains_key(&id) {
            return Err(InventoryError::DuplicateId);
        }
        self.items.insert(id, item);
        Ok(())
    }


    fn display_all(&self) -> String{
        let mut output = String::new();

        for (key, value) in &self.items {
            output.push_str(&format!(
                "Item Key: {}, Item Value: {}\n",
                key,
                value.display()
            ));
        }

        output
    }

    fn find_by_id(&self, id: String) -> Result<T, InventoryError> {
        match self.items.get(&id) {
            Some(item) => Ok(item.clone()),
            None => Err(InventoryError::ItemNotFound),
        }
    }
    
}



fn main() {

    println!("//--------------------------------------------- T as String -----------------------------------------------------");
    let mut i1: Inventory<String> = Inventory{
        items: HashMap::new()
    };
    let res1 = i1.add_item("1".to_string(), "Item 1".to_string());
    match res1 {
        Ok(()) => println!("Item added successfully"),
        Err(e) => println!("Error : {}",e),
    }
    let res2 = i1.add_item("2".to_string(), "Item 2".to_string());
    match res2 {
        Ok(()) => println!("Item added successfully"),
        Err(e) => println!("Error : {}",e),
    }

    let res3 = i1.add_item("".to_string(), "Item 1".to_string());
    match res3 {
        Ok(()) => println!("Item added successfully"),
         Err(e) => println!("Error : {}",e),
    }

    let res4 = i1.add_item("1".to_string(), "Item 1".to_string());
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
    println!("All Items: \n{}",i1.display_all());



    println!("//--------------------------------------------- T as Product -----------------------------------------------------");
     let mut i2: Inventory<Product> = Inventory{
        items: HashMap::new()
    }; 
    let  p1 = Product{id: 1, name: "Item1".to_string()};
    let  p2 = Product{id: 2, name: "Item2".to_string()};
    let res_p1 = i2.add_item("1".to_string(), p1);
    match res_p1{
        Ok(()) => println!("Item added successfully"),
        Err(e) => println!("Error : {}",e),
    }

    let res_p2 = i2.add_item("2".to_string(), p2);
    match res_p2{
        Ok(()) => println!("Item added successfully"),
        Err(e) => println!("Error : {}",e),
    }

    let res_p3 = i2.add_item("".to_string(), Product{id: 1, name: "Item1".to_string()});
    match res_p3{
        Ok(()) => println!("Item added successfully"),
        Err(e) => println!("Error : {}",e),
    }

    let res_p4 = i2.add_item("1".to_string(), Product{id: 1, name: "Item1".to_string()});
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
    println!("All Items: \n{}",i2.display_all());

}
