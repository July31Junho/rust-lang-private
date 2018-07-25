/*Random Numbers*/
extern crate rand;
use rand::Rng;

/*File Hnadling*/
use std::fs::File;
use std::io::prelude::*;

/*User Input*/
use std::io;

/*사용자 참여 시도*/
const ALLOWED_ATTEMPTS: u8 = 5;

/*문자 맞추기 구조*/
struct Letter {
    character: char,
    revealed: bool
}

/*게임 진행상황*/
enum GameProgress{
    InProgress,
    Won,
    Lost
}
fn main(){

    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = selected_word();
    let mut letters = create_letters(&selected_word);

    println!("Welcome to Hangman!");


    loop{
        println!("\n You have {} turns left.",turns_left);
        display_progress(&letters);

        println!("\n please enter a letter to guess:");
        let user_char = read_user_input_character();

        /*Exit if user enters an asterisk('*')*/
        if user_char == '*'{
            break;
        }

        /*Update the 'revealed' state of each letter. If the user
        has guessed a correct letter, at_least_one_revealed is changed
        to ture
        */

        let mut at_least_one_revealed = false;
        for letter in letters.iter_mut(){
            if letter.character == user_char{
                letter.revealed = true;
                at_least_one_revealed = true;
            }
        }
    /*Lose a turn if you make an incorrect guess*/
    if !at_least_one_revealed{
        turns_left -= 1;
    }

    /*Check game progress*/
    match check_progress(turns_left, &letters){
        GameProgress::InProgress => continue,
        GameProgress::Won => {
            println!("\n Congrats! you won! @");
            break;
        },
        GameProgress::Lost =>{
            println!("\n You lost !");
            break;
        }
    }
    }
    println!("\n Goob bye !");
}

/*Open the file containing list of words and select one at random,
returning it as a string 
*/
fn selected_word() -> String{
    let mut file = File::open("word.txt").expect("could not open file!");

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("An error occured while reading the file!");

    let available_words:Vec<&str> = file_contents.split(',').collect();

    /*Select word at random*/
    let random_index = rand::thread_rng().gen_range(0,available_words.len());
    return String::from(available_words[random_index]);
}

/*
    Given a word (type String), crate a vector of Letter's from it with default
    members and return it
*/
fn create_letters(word: &String) -> Vec<Letter>{
    let mut letters: Vec<Letter> = Vec::new();

    for c in word.chars(){
        letters.push(Letter{character:c,revealed:false});
    }

    return letters;
}

/*
    Display the progress of the game based off Vec<Letter>
    Example output : l _ n g _ _ g _ 
*/
fn display_progress(letters: &Vec<Letter>){
    //Example : Progress: _ a _ a _ y
    let mut display_string =String::from("Progress:");

    for letter in letters{
        display_string.push(' ');

        if letter.revealed{
            display_string.push(letter.character);
        }else{
            display_string.push('_');
        }

        display_string.push(' ');
    }
    println!("{}",display_string);

}

fn read_user_input_character() -> char {
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input){
        Ok(_) => {
            match user_input.chars().next(){
                Some(c) =>{return c;}
                None => {return '*';}
            }
        }
        Err(_) => {
            return '*';
        }
    }
}

/*
Checks the current state(Progress) of the game 
and returns the appropriate GameProgress member
*/
fn check_progress(turns_left:u8 , letters: &Vec<Letter>) -> GameProgress{
    /*Determine if all letters have been revealed*/
    let mut all_revealed = true;
    for letter in letters{
        if !letter.revealed{
            all_revealed = false;
        }
    }

    if all_revealed {
        return GameProgress::Won;
    }
    if turns_left > 0{
        return GameProgress::InProgress;
    }
    return GameProgress::Lost;
}









fn main() {
	panic_hook::set_abort();

	// the user has specified to run its originally installed binary (not via `parity-updater`)
	let force_direct = std::env::args().any(|arg| arg == "--force-direct");

	// absolute path to the current `binary`
	let exe_path = std::env::current_exe().ok();

	// the binary is named `target/xx/yy`
	let development = exe_path
		.as_ref()
		.and_then(|p| {
			p.parent()
				.and_then(|p| p.parent())
				.and_then(|p| p.file_name())
				.map(|n| n == "target")
		})
		.unwrap_or(false);

	// the binary is named `parity`
	let same_name = exe_path
		.as_ref()
		.map_or(false, |p| {
			p.file_stem().map_or(false, |n| n == PARITY_EXECUTABLE_NAME)
		});

	trace_main!("Starting up {} (force-direct: {}, development: {}, same-name: {})",
				std::env::current_exe().ok().map_or_else(|| "<unknown>".into(), |x| format!("{}", x.display())),
				force_direct,
				development,
				same_name);

	if !force_direct && !development && same_name {
		// Try to run the latest installed version of `parity`,
		// Upon failure it falls back to the locally installed version of `parity`
		// Everything run inside a loop, so we'll be able to restart from the child into a new version seamlessly.
		loop {
			// `Path` to the latest downloaded binary
			let latest_exe = latest_exe_path().ok();

			// `Latest´ binary exist
			let have_update = latest_exe.as_ref().map_or(false, |p| p.exists());

			// Canonicalized path to the current binary is not the same as to latest binary
			let canonicalized_path_not_same = exe_path
				.as_ref()
				.map_or(false, |exe| latest_exe.as_ref()
				.map_or(false, |lexe| exe.canonicalize().ok() != lexe.canonicalize().ok()));

			// Downloaded `binary` is newer
			let update_is_newer = latest_binary_is_newer(&latest_exe, &exe_path);
			trace_main!("Starting... (have-update: {}, non-updated-current: {}, update-is-newer: {})", have_update, canonicalized_path_not_same, update_is_newer);

			let exit_code = if have_update && canonicalized_path_not_same && update_is_newer {
				trace_main!("Attempting to run latest update ({})...",
							latest_exe.as_ref().expect("guarded by have_update; latest_exe must exist for have_update; qed").display());
				match run_parity() {
					Ok(_) => 0,
					// Restart parity
					Err(Error::Restart) => PLEASE_RESTART_EXIT_CODE,
					// Fall back to local version
					Err(e) => {
						error!(target: "updater", "Updated binary could not be executed error: {:?}. Falling back to local version", e);
						main_direct(true)
					}
				}
			} else {
				trace_main!("No latest update. Attempting to direct...");
				main_direct(true)
			};
			trace_main!("Latest binary exited with exit code: {}", exit_code);
			if exit_code != PLEASE_RESTART_EXIT_CODE {
				trace_main!("Quitting...");
				process::exit(exit_code);
			}
			trace!(target: "updater", "Re-running updater loop");
		}
	} else {
		trace_main!("Running direct");
		// Otherwise, we're presumably running the version we want. Just run and fall-through.
		process::exit(main_direct(false));
	}
}