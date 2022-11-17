enum Language {
    English, 
    Spanish, 
    Russian, 
    Japanese, 
}

fn main() {
    let language = Language::Japanese;
    
    match language {
        Language::English => println!("Hello from World!"), 
        Language::Spanish => println!("Hola Mundo!"), 
        Language::Russian => println!("Hi in Russian!"),
        _ => println!("Unsupported language!")
    }

    let authorization_status: Option<&str> = None; 
    let is_admin = false;
    let group_id:Result<u8, _> = "34".parse();

    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status);
    }else if is_admin {
        println!("Authorization status: Admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Authorization status: Privilaged");
        } else {
            println!("Authorization status: Basic");
        }

    }
}
