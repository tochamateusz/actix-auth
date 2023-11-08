use actix_web::{delete, get, post, HttpRequest, Responder};

#[derive(Debug)]
struct MyType {
    text: &'static str,
}

impl MyType {
    pub fn test(&self, var: &mut MyType) {
        var.text = &"dupa";
        println!("MyType {:?} {:?}", self, var)
    }
}

impl From<MyType> for String {
    fn from(my_type: MyType) -> Self {
        println!("Test {:?}", my_type.text);
        String::from(my_type.text)
    }
}

#[get("/users")]
pub async fn get_users(_req: HttpRequest) -> impl Responder {
    let my = MyType { text: "Get Users1" };
    let mut my2 = MyType { text: "Get Users2" };
    my.test(&mut my2);
    println!("get_users {:?}", my2);
    let text: String = my.into();
    text
}

#[get("/users/{id}")]
pub async fn get_users_by_id(_req: HttpRequest) -> impl Responder {
    "Tomek"
}

#[post("/users")]
pub async fn add_user(_req: HttpRequest) -> impl Responder {
    "Login"
}

#[delete("/users/{id}")]
pub async fn delete_user_by_id(_req: HttpRequest) -> impl Responder {
    "Delete user by id"
}

#[post("/login")]
pub async fn login(_req: HttpRequest) -> impl Responder {
    "Login"
}
