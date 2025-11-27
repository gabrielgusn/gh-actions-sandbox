#[macro_use] extern crate rocket;

// Test commit 2: Added for PR workflow validation

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test")]
fn test() -> &'static str{
    println!("Printing test");
    println!("done");
    return "Testing"
}

#[get("/new")]
fn new_endpoint() -> &'static str{
    "New endpoint"
}

#[get("/squad-1")]
fn squad_1() -> &'static str {
    "Squad commit 1"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, test, new_endpoint, squad_1])
}