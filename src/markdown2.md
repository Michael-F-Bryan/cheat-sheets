Markdoc2 Basics
===============

Creating a Wiki
---------------

Creating a wiki is usually fairly easy, once you've installed [markdoc2][md2],
you can run the `init` command with:

    markdoc2 init /path/to/wiki

(Note that the location can't already exist)

This will create a simple directory structure that looks something like this:

    /path/to/wiki
    ├── _html
    └── wiki 

The `/wiki` directory is where you will put all of the content for your wiki.
By default, the wiki builder can only render markdown files (ending in ".md").

All generated HTML will be put in the `/_html` directory, and the file structure
will closely mirror that of `/wiki`. The only exception being that all
directories will have a `index.html` file that acts as a table of contents for
everything in that directory, and all file extensions will be changed from
"md" to "html".


Editing Pages
-------------

Editing the wiki is just like editing any other markdown document. You use all
the standard markdown syntax, all links will be resolved to the documents they
point to (for internal links, make sure all are relative to the `/wiki`
directory and start with `/`), and you can even include code snippets like you
normally would.

Feel free to make sub-directories that nest as deep as you want, however
(similar to git), sub-directories will only be registered by `markdoc2` if they
contain wiki pages.

It is highly recommended to keep track of the wiki using some sort of version
control system, personally I prefer [git][git], but seeing as your wiki is just
composed of normal text files any VCS will work.


Viewing The Wiki
----------------

Because `markdoc2` will compile all of the wiki pages into normal html files
and resolve all links, once you have built the html (with `markdoc2 build`),
there is no extra software required for viewing your wiki. You can open the
files up in your favourite browser by just double clicking or right click and
"open with ...".


Publishing
----------

For the same reasons as in the last section, no extra software is required if
you want to publish your wiki on a server or the internet. Just use a normal
web server like [Nginx][nginx] or [Apache][apache], making sure to point it at
the `/_html` directory.

The easiest way to get your compiled html pages from your local computer to a
remote server is by using your normal [rsync][rsync] commands.


Performance
-----------

Generally, the `markdoc2 build` command is very fast and only needs to be run
infrequently anyway. As a test, I created 100 basic markdown files with some
simple content (totalling about 500 KB of data) and just using the built in
`time` command it took 0.97 seconds to build the entire wiki on my laptop. Your
mileage may vary.

Later on, I'll probably implement incremental build functionality, so only the
pages that have been changed are recompiled, but for now every time you run
`markdoc2 build` it'll recompile the entire wiki.


Usability
---------

Purely for convenience sake, I decided to write a super simple Makefile to keep
track of the various commands I use when managing the wiki. The Makefile looks
something like this:

    MARKDOC = markdoc2
    HTML_DIR = _html

    # Production stuff
    PROD_USER = XXXX
    PROD_SERVER = XXXX
    PROD_LOCATION = /var/www/html
    RSYNC_ARGS = --archive --delete --progress 


    view: 
        $(MARKDOC) build -b

    build:
        $(MARKDOC) build

    watch:
        $(MARKDOC) watch -b

    publish:
        rsync $(RSYNC_ARGS) $(HTML_DIR) $(PROD_USER)@$(PROD_SERVER):$(PROD_LOCATION)

    .PHONY: build view watch publish

In particular, the `markdoc2 watch -b` command is super useful. What it will do
is build your wiki, then immediately open up the wiki root in your browser, and
finally it'll watch your entire `/wiki` directory for any changes and rebuild
the wiki as necessary. This means you can be editing a page (or create a new
one) in your browser and every time you hit save all you need to do is refresh
your browser window and you'll see all the changes.

Note that `markdoc2 watch` relies on a Linux kernel feature called `inotify`.
Basically you tell the kernel what files you want to watch and the events to
watch for (file created/edited etc), and it'll notify you when those events 
happen. 

If I were to implement this for Windows or OSX, it'd require checking every x
seconds to see if there were any changes (polling), not only is this quite
inefficient, but it's not a very elegant thing either. So for now, `markdoc2
watch` is only available on Linux.


[md2]: https://github.com/Michael-F-Bryan/markdoc2
[git]: https://git-scm.com/
[nginx]: https://nginx.org/en/
[apache]: http://www.apache.org/
[rsync]: https://www.digitalocean.com/community/tutorials/how-to-use-rsync-to-sync-local-and-remote-directories-on-a-vps
