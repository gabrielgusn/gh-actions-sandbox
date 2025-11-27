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

#[get("/after-merge")]
fn after_merge() -> &'static str {
    "This is the after merge endpoint"
}

#[get("/after-merge2")]
fn after_merge_2() -> &'static str {
    "This is the after merge 2 endpoint"
}

#[get("/after-merge3")]
fn after_merge_3() -> &'static str {
    "This is the after merge 3 endpoint"
}

#[get("/after-merge4")]
fn after_merge_4() -> &'static str {
    "This is the after merge 4 endpoint"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, test, new_endpoint, squad_1, after_merge, after_merge_2, after_merge_3, after_merge_4])
}