mod modules;
fn main() {
    println!("----------------------Control flow----------------------");
    modules::control_flow::run();
    println!("----------------------Structs and methods----------------------");
    modules::structs_and_methods::run();
    println!("----------------------Serde Serialization----------------------");
    modules::serde_serialization::run();
    println!("----------------------Serde Deserialzation----------------------");
    modules::serde_deserialization::run();
    println!("----------------------Request Traxking----------------------");
    modules::req_track::run();
}
