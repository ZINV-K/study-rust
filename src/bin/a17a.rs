// ? - Result, Ok, Err
#[derive(Debug)]
enum Menu {
	Main,
	Start,
	Quit,
}

fn get_choice(input: &str) -> Result<Menu, String> {
	match input {
		"main" => Ok(Menu::Main),
		"start" => Ok(Menu::Start),
		"quit" => Ok(Menu::Quit),
		_ => Err("There is no menu like that".to_owned()),	
	}
}

fn print_choice(choice: &Menu) {
    println!("processing...");
	println!("choice: {:?}", choice);
}

fn pick_choice(input: &str) -> Result<(), String> {
	let choice = get_choice(input)?; // <--- Ok() 안에 있는 값만 가져옴
	print_choice(&choice);
	Ok(())
}

fn main() {
	let choice = pick_choice("leave");
    println!("choice is {:?}", choice);
	// let choice = Result<Menu, _> = get_choice("leave");

	// match choice {
	// 	Ok(value) => get_choice(&value),
	// 	Err(e) -> println!("Error: {:?}", e),	
	// }
}