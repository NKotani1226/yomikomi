fn main() {
    let url = "https://www.rust-lang.org";
    let rc = reqwest::blocking::get(url).unwrap(); //同期処理にするためreqwest::blockingから呼び出す。
    let contents = rc.text().unwrap();
    println!("{}", contents);
}