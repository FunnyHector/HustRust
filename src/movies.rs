// any function inside a mod is private by default.
// use the keyword `pub` to make it publicly available.
pub fn play(name: String) {
    println!("Playing movie {}", movie_name_with_year(name));
}

// this is a private function, only available inside movies or a descendant
fn movie_name_with_year(name: String) -> String {
    name.to_owned() + "(2008)" // hardcoded year, demonstration only :)
}
