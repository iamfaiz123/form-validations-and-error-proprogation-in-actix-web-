#[allow(deprecated)]
use actix_web::{get, web, App, HttpServer};
use form_validator::api_errors::ApiErrors;
use form_validator::forms::Form;
use validator::Validate;

#[get("/")]
async fn hello(form: web::Json<Form>) -> Result<&'static str, ApiErrors> {
    match form.into_inner().validate() {
        Ok(_) => {}
        Err(e) => return Err(ApiErrors::Input(e)),
    }
    //return

    Ok("forms validation worked fine")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
