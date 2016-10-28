#![feature(proc_macro)]

extern crate tera;
extern crate lettre;

use tera::{Tera, TeraResult, Context};
use lettre::email::EmailBuilder;

fn main() {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let templates = format!("{}/*.txt", project_root);
    let tera = Tera::new(&templates);

    let sender_name = "Michael";
    let sender_email = "michael@pureawesome.io";
    let clients = vec![
        ("John", "john@example.org", 1),
        ("Mary", "mary@foobar.com", 4),
    ];

    let mut emails = vec![];

    for (client_name, client_email, years_as_client) in clients {
        let text = format_email(&tera, client_name, years_as_client, sender_name).unwrap();

        let email = EmailBuilder::new()
            .to((client_email, client_name))
            .from(sender_email)
            .subject("Hi, Hello world")
            .text(&text)
            .build()
            .unwrap();

        emails.push(email);
    }
}



fn format_email(tera: &Tera, name: &str, years_as_client: u32, sender: &str) -> TeraResult<String> {
    let normalized_duration = if years_as_client == 1 {
        String::from("1 year")
    } else {
        format!("{} years", years_as_client)
    };

    let mut context = Context::new();
    context.add("name", &name);
    context.add("years_as_client", &normalized_duration);
    context.add("sender", &sender);

    tera.render("email.txt", context)
}
