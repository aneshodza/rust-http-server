/// Prints a greeting using ASCII-Art to the console. Print this at the start of your program.
pub fn greet_user() {
    println!(r#"
    __    __  __                                            
   / /_  / /_/ /_____        ________  ______   _____  _____
  / __ \/ __/ __/ __ \______/ ___/ _ \/ ___/ | / / _ \/ ___/
 / / / / /_/ /_/ /_/ /_____(__  )  __/ /   | |/ /  __/ /    
/_/ /_/\__/\__/ .___/     /____/\___/_/__  |___/\___/_/     
        ____ /_/  ____ _____/ /__     / /_  __  __          
       / __ `__ \/ __ `/ __  / _ \   / __ \/ / / /          
      / / / / / / /_/ / /_/ /  __/  / /_/ / /_/ /           
     /_/ /_/ /_/\__,_/\__,_/\___/  /_.___/\__, /            
                            __           /____/             
  ____ _____  ___  _____   / /_  ____  ____/ /___  ____ _   
 / __ `/ __ \/ _ \/ ___/  / __ \/ __ \/ __  /_  / / __ `/   
/ /_/ / / / /  __(__  )  / / / / /_/ / /_/ / / /_/ /_/ /    
\__,_/_/ /_/\___/____/  /_/ /_/\____/\__,_/ /___/\__,_/     

    "#);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user_throws_no_errors() {
        greet_user()
    }
}
