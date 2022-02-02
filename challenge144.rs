use std::io;

fn rot13(cipher: String) -> String {
    let mut plain_text = String::new();
    let offset = 13;
    println!("Deciphering...");
    for c in cipher.chars() {
        if !c.is_alphabetic() {
            plain_text.push(c);
            println!("Decrypted {} as {}", c, c);
            continue;
        }
        let cipher_ascii_value = c as u8;
        let decrypted_ascii_value = cipher_ascii_value + offset;
        let decrypted_ascii_value = {
            if c.is_uppercase() {
                if decrypted_ascii_value > 'Z' as u8 {
                    'A' as u8 + decrypted_ascii_value - 'Z' as u8 - 1
                } else {
                    decrypted_ascii_value
                }
            } else {
                if decrypted_ascii_value > 'z' as u8 {
                    'a' as u8 + decrypted_ascii_value - 'z' as u8 - 1
                } else {
                    decrypted_ascii_value
                }
            }
            
        };
        let pc = decrypted_ascii_value as char;
        println!("Decrypted {} as {}", c, pc);
        plain_text.push(pc);
    }

    return plain_text
}

fn main() {
    print!("Enter your cipher: ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse::<String>().unwrap();
    let cipher = buffer;
    let plain_text = rot13(cipher);

    println!("Plain text: {}", plain_text);
}