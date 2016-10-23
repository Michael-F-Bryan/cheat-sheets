Michael's Cheat Sheets
======================

A collection of various cheat sheets, code snippets and quick articles
explaining some of the common things I've done.


Usage
-----

To generate the pretty html version, you need to have `mdbook` installed.
`mdbook` is a gitbook-like program written in the `Rust` programming language
mainly targeted at marking up and presenting documentation. To install it you
can do

    cargo install mdbook

Then generating the html is as simple as `cd`-ing to this root directory and
running

    mdbook build

If you don't have `Rust` installed, by far the easiest thing to do is get it
from [https://rustup.rs/][rustup]. `Rustup` is a tool created by the `Rust`
developers for managing multiple versions of the `Rust` compiler at the same
time, as well as continuously keeping it up to date, and installing alternate
compilation targets (e.g. so you can effortlessly cross-compile for Android or
Windows from a Linux machine).

`mdbook` also has a nice feature where it'll serve your documentation and as
you are editing it, everything will get regenerated and updated in your browser
every time a file is saved.

    mdbook serve

`mdbook` expects a directory structure similar to the following:

    .
    ├── README.md
    ├── src
    |  ├── SUMMARY.md
    |  ├── chapter-1.md
    |  └── ...
    └── book
       ├── index.html
       └── ...

When you build the book, all generated html is put in the `/book` directory
using the Markdown files in the `/src` directory according to the contents of
`/src/SUMMARY.md`.

For more information on how to use `mdbook`, consult the
[documentation][mdbook].


[mdbook]: http://azerupi.github.io/mdBook/README.html
[rustup]: https://rustup.rs/
