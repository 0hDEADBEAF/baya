fn main() {
    const RE_JSON: &str = include_str!(concat!(env!("OUT_DIR"), "/re.json"));

    println!("{}", RE_JSON);
}
