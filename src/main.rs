use chumsky::prelude::*;

fn parse_number() {
    text::int(10).collect::<String>().from_str().unwrapped()
}

fn main() {}
