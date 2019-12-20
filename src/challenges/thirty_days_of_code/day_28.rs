// 28. RegEx, Patterns, and Intro to Databases
// https://www.hackerrank.com/challenges/30-regex-patterns

use crate::functions;

struct Contact {
    name: String,
    email: String,
}

pub fn main() {
    let mut number_contacts: u32 = match String::from(functions::read_line().trim()).parse::<u32>()
    {
        Ok(n) => n,
        Err(e) => {
            println!("{}", e);
            0
        }
    };

    let mut gmail_accounts: Vec<String> = vec![];

    while number_contacts > 0 {
        let contact_input: Vec<String> = String::from(functions::read_line().trim())
            .split(' ')
            .into_iter()
            .map(|s| s.parse::<String>())
            .collect::<Result<Vec<String>, std::convert::Infallible>>()
            .unwrap();
        let contact = Contact {
            name: contact_input[0].clone(),
            email: contact_input[1].clone(),
        };
        if contact.email.ends_with("@gmail.com") {
            gmail_accounts.push(contact.name);
        }
        number_contacts -= 1;
    }
    gmail_accounts.sort();
    println!("{}", gmail_accounts.join("\n"));
}
