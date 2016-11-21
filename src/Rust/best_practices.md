# Best Practices

Here's a bunch of best practices and design patterns I've discovered in my time
writing Rust code.

A lot of these points come from [this awesome article][article], so I'd
recommend checking that out too.

## Quick Links

For those too lazy to scroll:

* [Format Your Code](Rust/best_practices.html#Format%20Your%20Code)
* [Document All The Things!](Rust/best_practices.html#Document%20All%20the%20Things!)
* [Add Even More Lints](Rust/best_practices.html#Add%20Even%20More%20Lints)
* [Miscellaneous Tips](Rust/best_practices.html#Miscellaneous%20Tips)


## Format Your Code

It's always nice to have your code formatted correctly and when everyone follows
a common coding standard, it's even nicer when your computer can enforce all
this for you. You should probably use [rustfmt][rustfmt] for this...

```
$ cargo install rustfmt
```

By default, this also installs a `fmt` sub-command for use with `cargo`, so now
you can reformat your entire library with a simple `cargo fmt` call.

If you're using the `Atom` editor and have `rustfmt` installed then it can run
`rustfmt` every time you hit save using [atom-beautify][ab].

```
$ apm install atom-beautify
```

To configure `rustfmt`, just drop a file called `rustfmt.toml` (or
`.rustfmt.toml` if you want) with a bunch of configuration options. You can get
the list of valid configuration options from rustfmt itself.

```
rustfmt --config-help
```

This is what my usual `.rustfmt.toml` looks like:

```
$ cat .rustfmt.toml
format_strings = false
reorder_imports = true
take_source_hints = false
report_todo = "Always"
report_fixme = "Always"
```


## Document All the Things!

It's always nice when you can flick through a crate's documentation and tell
straight away how to use it and what everything does. Therefore as a bare
minimum you should make sure that **all** exported functions, structs, and enums
have at least a line or two describing what they do.

To enforce this, you can add a lint to your root `lib.rs` to prevent any changes
which add undocumented public interfaces.

```
#![deny(missing_docs)]
```

Next up, for any major function or struct, make sure you add a couple of
examples in the doc comment. These are also compiled and run when you run your
test suite (you do have tests, don't you?) so it's easy to keep them up to date.

```
/// (summary line here)
///
/// some more info about the function...
///
/// # Examples
///
/// ```
/// let src = "(print 1 2)";
/// let ast = parse(src).unwrap();
/// ```
```

## Add Even More Lints

You can get the compiler to do even more checking for you by simply adding a
couple extra lints to your crate.

```
#![deny(missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    unused_results,
    variant_size_differences)]
```


## Continuous Integration

If you use GitHub for storing your code then adding CI is a fairly trivial thing
to do. Here's a detailed example `.travis.yml` which will automatically test
your library on all three channels of the rust compiler, then generate and
upload your crate's documentation to [GitHub Pages][ghp].

```
$ cat .travis.yml
sudo: false
language: rust
rust:
- nightly
- beta
- stable
matrix:
allow_failures:
- rust: nightly
before_script:
- |
pip install 'travis-cargo<0.2' --user &&
export PATH=$HOME/.local/bin:$PATH
script:
- |
travis-cargo build &&
travis-cargo test &&
travis-cargo bench &&
travis-cargo --only stable doc
after_success:
- travis-cargo --only stable doc-upload
env:
global:
- TRAVIS_CARGO_NIGHTLY_FEATURE=dev
- secure: # encrypted stuff
```

To enable the auto­matic doc­u­men­ta­tion uploads (i.e. push­ing rust­doc's out­put to
the pro­jec­t's `gh-pages` branch) sup­ported by `travis-cargo`, you need to add an
envi­ron­ment vari­able called `GH_TOKEN` that con­tains an access token for your
GitHub account (encrypted and with lim­ited rights). You can cre­ate one
[here][token].

To encrypt the token, you can use [Trav­is' CLI tool][cli] (installed with
`gem install travis`) by run­ning this in your pro­jec­t's root direc­tory
(replace "1234" with your token):

```
$ travis encrypt "GH_TOKEN=1234" --add env.global
```

When every­thing is set up cor­rectly, you should be able to view your pro­jec­t's
doc­u­men­ta­tion at `https://$USERNAME.github.io/$PROJECT`.


## Error Handling

For non-trivial crates where you may have several common error cases, you might
want to define your own error type. This type can then implement
[std::convert::From][from] for all the errors you may be wrapping. That way the
error integrates nicely with the `try!()` macro and your entire library will
be simplified by using only one error and `Result` type.

```rust
use std::io;
use std::num;

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug)]
enum CliError {
  Io(io::Error),
  Parse(num::ParseIntError),
}

impl From<io::Error> for CliError {
  fn from(err: io::Error) -> CliError {
    CliError::Io(err)
  }
}

...
```

You should also make sure your custom error implements the `std::error::Error`
trait.

```rust
impl Error for CliError {
  fn description(&self) -> &str {
    match *self {
      CliError::Io(ref err) => err.description(),
      CliError::Csv(ref err) => err.description(),
    }
  }

  fn cause(&self) -> Option<&Error> {
    ...
  }
}
```


## Miscellaneous Tips

* Don't dupli­cate your bina­ry's ver­sion num­ber in your code. Use
  `env!("CARGO_PKG_VERSION")` to get the ver­sion you set in `Cargo.toml` at
com­pile time.
* Write com­mit mes­sages in a [“con­ven­tional”][style] style and use
  [Clog][clog] to auto­mat­i­cally gen­er­ate change logs.
* Install more [Cargo Sub­com­mands][cargo]!
* Make sure that all you test each feature of your code (note "unit" in "unit
  test" doesn't necessarily mean each and every little function).
* Integration tests are awesome too.


[article]: https://pascalhertleif.de/artikel/good-practices-for-writing-rust-libraries/
[rustfmt]: https://github.com/rust-lang-nursery/rustfmt
[ab]: https://atom.io/packages/atom-beautify
[ghp]: https://pages.github.com/
[token]: https://github.com/settings/tokens
[cli]: https://github.com/travis-ci/travis.rb
[style]: https://github.com/ajoslin/conventional-changelog/blob/4bf7d8df29f7be29552b82ec2b7112fad0c39a3e/conventions/angular.md
[clog]: https://github.com/clog-tool/clog-cli
[cargo]: https://github.com/rust-lang/cargo/wiki/Third-party-cargo-subcommands
