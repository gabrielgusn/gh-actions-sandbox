#[macro_use] extern crate rocket;

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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, test])
}