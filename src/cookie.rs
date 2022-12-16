use actix_web::{cookie::Cookie, HttpRequest};

pub fn get_cookie_string_from_header(req: HttpRequest) -> Option<String> {
    let cookie_header = req.headers().get("cookie");
    if let Some(v) = cookie_header {
        let cookie_string = v.to_str().unwrap();
        return Some(String::from(cookie_string));
    }

    None
}

pub fn get_cookie_value(_key: &str, cookie_string: String) -> Option<String> {
    let kv: Vec<&str> = cookie_string.split(';').collect();
    for c in kv {
        match Cookie::parse(c) {
            Ok(kv) => {
                if "token" == kv.name() {
                    return Some(String::from(kv.value()));
                }
            }
            Err(e) => {
                println!("cookie parse error -> {}", e);
            }
        }
    }

    None
}

pub fn set_cookie(token: &str) -> Cookie {
    Cookie::build("token", token)
        .secure(true)
        .http_only(true)
        .finish()
}
