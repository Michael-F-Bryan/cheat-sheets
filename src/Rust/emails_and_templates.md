# Emails and Templates #

Consider this hypothetical situation; you're at work and your boss walks up to
you saying you need to send out emails to each of your (100+) clients, and
they all need to be personalised for marketing purposes.

It turns out that this is actually a pretty easy job to do in Rust. Normally
Rust is a systems programming language, but it has enough zero cost abstractions
and high level features that templating and emails in Rust are about as easy
to do as they are in Python.

For this exercise, we'll be needing two crates:

- [lettre][lettre] for sending emails, and
- [tera][tera] for Django/Jinja2 style templating

First create a new executable,

    cargo new --bin spammer

And add the dependencies to your `Cargo.toml`,

```
[dependencies]
tera = "*"
lettre = "*"
```


## Writing Templates ##

If you've ever done templating in Python before, the template syntax will be
pretty familiar. Basically, when you render the template you'll pass it a
`Context` (essentially just a dictionary) that contains all the variables and
data you want to substitute into your template.

The template itself is just a file that sits on disk somewhere.

```
$ cat ./emails.txt
Hello {{ name }},

You have been a client of ours for {{ years_as_client }}, thank you for
choosing us and we hope to continue having a great relationship into the future.

Kind Regards,
{{ sender }}
```

Creating a context and populating it isn't too difficult. You just call the
`add()` method and give it the key, plus a pointer to whatever you're wanting to
substitute in.

```rust
let mut context = Context::new();
context.add("name", &name);
context.add("years_as_client", &normalized_duration);
context.add("sender", &sender);
```

You also need to create a `Tera` object. Think of this as just a rendering
engine. It needs to know which templates you're wanting to use (I'm using the
`env!()` macro here to embed the location of the project directory at compile
time).

```rust
let project_root = env!("CARGO_MANIFEST_DIR");
let templates = format!("{}/*.txt", project_root);
let tera = Tera::new(&templates);
```

Then rendering the templates is as simple as iterating through your clients and
telling `tera` to render the template. To make life easier, I pulled the
template construction out into its own function.

```rust
fn main() {
    ...

    for (name, years_as_client) in clients {
        let text = format_email(&tera, name, years_as_client, sender).unwrap();
        println!("-----");
        println!("{}", text);
    }

    ...
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
```


## Sending Emails ##

Constructing and sending emails using [lettre][lettre] is just as easy.

First you'll construct the email (copied straight from their docs):

```rust
use lettre::email::EmailBuilder;

// Create an email
let email = EmailBuilder::new()
    // Addresses can be specified by the tuple (email, alias)
    .to((client_email, client_name))
    .from(sender_email)
    .subject("Hi, Hello world")
    .text(text)
    .build();

assert!(email.is_ok());
```

Sending emails is also fairly easy, here's a fairly detailed example (again,
pulled straight from the docs):

```rust
use lettre::email::EmailBuilder;
use lettre::transport::smtp::{SecurityLevel, SmtpTransport,
SmtpTransportBuilder};
use lettre::transport::smtp::authentication::Mechanism;
use lettre::transport::smtp::SUBMISSION_PORT;
use lettre::transport::EmailTransport;

let email = EmailBuilder::new()
                    .to("root@localhost")
                    .from("user@localhost")
                    .body("Hello World!")
                    .subject("Hello")
                    .build()
                    .unwrap();

// Connect to a remote server on a custom port
let mut mailer = SmtpTransportBuilder::new(("server.tld",
SUBMISSION_PORT)).unwrap()
    // Set the name sent during EHLO/HELO, default is `localhost`
    .hello_name("my.hostname.tld")
    // Add credentials for authentication
    .credentials("username", "password")
    // Specify a TLS security level. You can also specify an SslContext with
    // .ssl_context(SslContext::Ssl23)
    .security_level(SecurityLevel::AlwaysEncrypt)
    // Enable SMTPUTF8 if the server supports it
    .smtp_utf8(true)
    // Configure expected authentication mechanism
    .authentication_mechanism(Mechanism::CramMd5)
    // Enable connection reuse
    .connection_reuse(true).build();

let result_1 = mailer.send(email.clone());
assert!(result_1.is_ok());

// The second email will use the same connection
let result_2 = mailer.send(email);
assert!(result_2.is_ok());

// Explicitly close the SMTP transaction as we enabled connection reuse
mailer.close();
```

[lettre]: https://github.com/lettre/lettre
[tera]: https://github.com/Keats/tera
