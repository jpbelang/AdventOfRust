use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_one_elf(reader: &mut dyn BufRead) -> u32 {
    let mut total = 0;

    loop {
        let mut line: String = String::new();
        let result = reader.read_line(&mut line);
        if result.expect("reading error!") == 0 || line.trim().len() == 0 {
            return total;
        } else {
            let q = line.trim().parse::<u32>().expect("invalid number");
            println!("{q}");
            total += q;
        }
    }
}

fn read_all_elves(reader: &mut dyn BufRead) -> Vec<u32> {
    let mut return_vector = Vec::new();
    loop {
        let v = read_one_elf(reader);
        if v > 0 {
            return_vector.push(v)
        } else {
            return return_vector;
        }
    }
}

fn main() {
    let buf = BufReader::new(File::open("test1.txt").expect("file not fount"));
    let elf = read_all_elves(&buf);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn read_one_elf_only() {
        let single_list = r#"1000
2000
3000"#;
        let elf = read_one_elf(&mut single_list.as_bytes());
        assert_eq!(elf, 6000)
    }

    #[test]
    fn read_one_elf_only_stop_empty_line() {
        let single_list = r#"1000
2000
3000

1000
"#;
        let elf = read_one_elf(&mut single_list.as_bytes());
        assert_eq!(elf, 6000)
    }

    #[test]
    fn read_all_elves_test() {
        let single_list = r#"1000
2000
3000

1000
"#;
        let elves: Vec<u32> = read_all_elves(&mut single_list.as_bytes());
        assert_eq!(elves.len(), 2);
        assert_eq!(elves[0], 6000);
        assert_eq!(elves[1], 1000);
    }
}
