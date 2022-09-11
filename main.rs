fn replace_nth_char_safe(s: &str, idx: usize, newchar: char) -> String {
    s.chars().enumerate().map(|(i,c)| if i == idx { newchar } else { c }).collect()
}

fn convert(c: char) -> char {
    if c == 'A' {return 'C'};
    if c == 'C' {return 'G'};
    if c == 'G' {return 'T'};
    if c == 'T' {return 'A'};
    return ' ';
}

fn main() {
    println!("Start");

    let opt = String::from("ACGT");
    let mut s = String::from("");
    let mut s_last = String::from("");
    let len_str = 13;
    let mut change_next: bool;

    for _ in 1..=len_str {
        s.push(opt.chars().nth(0).unwrap());
    }

    for _ in 1..=len_str {
        s_last.push(opt.chars().rev().nth(0).unwrap());
    }

    let mut counter = 1;
    while s != s_last {
        counter = counter + 1;
        change_next = true;

        for i in 0..=len_str-1 {
            let ith_char = s.chars().nth(i).unwrap();
            if change_next {
                if ith_char == opt.chars().rev().nth(0).unwrap() {
                    s = replace_nth_char_safe(&s, i, convert(ith_char));
                    change_next = true;
                } else {
                    s = replace_nth_char_safe(&s, i, convert(ith_char));
                    break;
                }
            }
        }
    }
    // You can uncomment the next line to see all k-mers.
    // print(s)
    println!("Number of generated k-mers: {}", counter);
    println!("Finish!");
}