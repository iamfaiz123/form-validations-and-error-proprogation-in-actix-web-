pub mod api_errors {
    use actix_web::{
        http::{header::ContentType, StatusCode},
        HttpResponse,
    };

    //validate crates provide inbuild functions to validate form , #[validate(email)] is an example, using custom argument in macro we can
    // use custom validation functions #[validate(length(min = 1), custom = "validate_unique_username")]

    use validator::ValidationErrors;
    //define error
    #[derive(Debug)]
    pub enum ApiErrors {
        //other errors can be added and implemented
        Input(ValidationErrors),
    }

    impl core::fmt::Display for ApiErrors {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self {
                //match at which filed the error occurred
                ApiErrors::Input(e) => {
                    if ValidationErrors::has_error(&Err(e.clone()), "email") {
                        write!(f, "invalid email address")
                    } else if ValidationErrors::has_error(&Err(e.clone()), "first_name") {
                        write!(f, "invalid name , it must not contains special characters")
                    } else {
                        write!(f, "oops! something went wrong with validations of inputs")
                    }
                }

                _ => write!(f, "oops! No idea"),
            }
        }
    }

    impl actix_web::error::ResponseError for ApiErrors {
        fn error_response(&self) -> HttpResponse {
            HttpResponse::build(self.status_code())
                .insert_header(ContentType::html())
                .body(self.to_string())
        }

        fn status_code(&self) -> StatusCode {
            StatusCode::BAD_REQUEST
        }
    }
}

pub mod forms {
    use crate::utils::validation_functions::is_sql_vulnerable;
    use serde::Deserialize;
    use validator::Validate;
    #[derive(Debug, Deserialize, Validate)]
    pub struct Form {
        //making each field as public to easy access
        #[validate(email)]
        pub email: String,
        //for hashpassword we will be stored sha256 hash
        pub hashed_password: String,
        #[validate(length(min = 1), custom = "is_sql_vulnerable")]
        pub first_name: String,
    }
}

pub mod validation_functions {
    use regex::bytes::Regex;
    use validator::ValidationError;

    pub fn is_sql_vulnerable(input: &str) -> Result<(), ValidationError> {
        let re = Regex::new(r#"([%_\\])|(--[^\r\n]*)|(/\*[\s\S]*?\*/)"#).unwrap();
        if re.is_match(input.as_bytes()) {
            Err(ValidationError::new(""))
        } else {
            Ok(())
        }
    }
}
