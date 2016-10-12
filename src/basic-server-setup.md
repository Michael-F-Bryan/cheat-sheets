Basic Steps For Setting Up A Server
===================================

I thought I'd make a post to help remind myself of some of the important things
to do when securing a new computer, I'll try to explain some of the reasoning
behind some of it as well. For both your benefit and mine.

For the sake of this post, I'm going to assume you're setting up a new Ubuntu
server. You should try to do this entire process in one hit, that way you'll
always be using the same SSH session and won't get accidentally locked out.

## TL;DR

- Set proper account access and Passwords.
- Configure a firewall and make it as restrictive as possible while still
  allowing the server to do its job.
- Set up SSH properly. Allow key-only access and no root logins.
- Install something like `logwatch` for basic monitoring.


## What Do We Need?

Instead of starting off installing random programs and configuring them, it's
better to list some of the things we'll need first. So what will we need to do?

Accounts and Passwords
:    Sounds like a pretty basic thing, yet 9 times out of 10 the reason you get
     hacked probably won't be because of some obscure software vulnerability,
     but because you left the root account's password as "password" or "root".

Firewall
:    For blocking off all ports (and therefore services) from being accessible
     from the wider internet, with the exception of a chosen few (e.g. ssh and
     http). We'll also implement rate limiting here so people making too many
     connections too quickly will be blocked.

SSH Configuration
:   Configure SSH so that you can't SSH in as root. While we're at it, we may
    as well make it so you can only log in if you have the correct SSH key.


## Accounts and Passwords

The first time you log into your new computer, you'll want to create your own
user account straight away and give it super-user access via `sudo`. The reason
you'll want to do this instead of setting everything up as root is that being
forced to type `sudo` before commands that require super-user access is a
good way to remind yourself that whatever you're changing will affect your
entire computer, not just your account.

To add a user, bob:

    adduser bob

Then follow the prompts.

To give the user the ability to use sudo, you'll need to add them to the `sudo`
group:

    adduser bob sudo

When choosing your user's password, it's important that you make sure it's
reasonably secure. As a general rule of thumb, it should incorporate a
**minimum** of 8 characters, with 1 or more number and with mixed case.

The reason I'm stressing this point is that on one of the machines I've had in
the past I gave the `git` user the highly original password, "password",
because I was using it as a private git repository and wanted to be able to add
new repositories easily. As you can probably guess, after a while someone
figured this out, most probably through trial and error (or dumb luck), and
then used the server (with it's insane internet speeds) to DOS people. Luckily
after maybe an hour of this excessively high internet usage, the company
hosting my server realised something was out of the ordinary and shut it down,
but things could have ended up a lot worse.

So yeah, weak passwords aren't a great idea. Even if you make sure the account
has very limited abilities.

Once you've created your new user and given him sudo access, login to that
account:

    login bob


## Firewalls

The next step in securing our server is to install a firewall. Think of a
firewall as the gatekeeper for your computer, it'll block any incoming packets
to all ports except the few you choose to leave open (usually just SSH and
HTTP). You can also get the firewall to block certain IP addresses or only
selectively allow access to ports.

A good example of this is how all of my home computers allow SSH access only if
you are on the local network. This means that someone trying to SSH in from
outside (i.e. a hacker or script kiddie) will just be blocked, while you can
still SSH in from the other room.

The particular firewall we're going to use is the aptly named [Uncomplicated
Firewall][ufw]. It is essentially just a wrapper around the more complicated
`iptables` which comes with almost every Linux distro.

First you need to install UFW:

    sudo apt-get install ufw

Then you'll want to enable it:

    sudo ufw enable

You can then view the default rules if you want:

    sudo ufw status

Or, alternatively:

    sudo ufw status

Now that you've got UFW installed and enabled, you'll want to create some rules
for it. Depending on your particular circumstances, the rules you create will
differ but at a bare minimum you'll want to allow SSH to pass through the
firewall. Otherwise how are you going to log into your server later on?

The syntax for adding rules is fairly straightforward:

    sudo ufw allow in ssh

You can specify the port either with a service name (i.e. "ssh"), or its
number (i.e. 22).

UFW also has the ability to do `rate limiting`. To deny connections from an
IP address that has attempted to initiate 6 or more ssh connections in the last
30 seconds, you can do this:

    sudo ufw limit ssh

A slightly more complicated example is only allowing someone to SSH in from the
local network (for example using CIDR notation, mine is 192.168.0.0/16).

    sudo ufw allow from 192.168.0.0/16 to 0.0.0.0 port 22

The "0.0.0.0" above is just a shortcut for "all interfaces for this
computer".


## Lock Down SSH

By default SSH isn't configured in the most secure way. For instance, you are
allowed to log in as root, and you only need a password in order to log in as
any other user.

The days of passwords are over. You’ll enhance security and ease of use in one
fell swoop by ditching those passwords and employing public key authentication
for your user accounts.

SSH keys work by generating a pair of very long numbers, one is called the
`public key` and is given to whoever you want to contact, and the other is
called the `private key`, you're the only person who should ever have access to
this one. When you attempt to log in using SSH keys, the other computer will
send a message encrypted with your public key that can only be decrypted with
your private key.

The keys themselves are many times longer than your password, often thousands
of bits long, which means it's a lot less likely that someone can pretend to be
you by brute-forcing the keys. It also relies on having a specific file on your
computer which is stored in a place that only you should be able to access,
which is theoretically a lot safer and more reliable than remembering a
password.

Additionally, (and probably most importantly) it means you won't need to type
in a complicated password every time you want to log into a computer. Making
your life easier and also allowing you to incorporate SSH into your scripts.

To generate a set of SSH keys run the following and follow the prompts:

    ssh-keygen

The defaults are usually good enough for most purposes.

Next you'll want to give your server your computer's public key. Do so by
creating a new file,

    vim ~/.ssh/authorized_keys

And copy/pasting your public key (`~/.ssh/id_rsa.pub` by default) in at the
bottom.

Next make sure the authorized keys file has the correct permissions. If you
don't do this, then SSH will assume they've been tampered with and lock you
out.

    chmod 400 /home/bob/.ssh/authorized_keys
    chown bob:bob /home/bob/.ssh -R

The next step is to actually go into the SSH configuration file and change
things so that you can't SSH in as root and so that SSH keys are required.

    vim /etc/ssh/sshd_config


Add these lines to the file, inserting the ip address from where you will be
connecting:

    PermitRootLogin no
    PasswordAuthentication no

If you know you'll only be logging in from one location (e.g. your workplace),
you can further strengthen security by locking SSH to your particular IP.

    AllowUsers bob@(your-ip) bob@(another-ip-if-any)

Now finally, restart the SSH service with

    sudo service ssh restart

Or if you're using Systemd:

    sudo systemctl restart ssh


## Conclusion

You should now have most of the basics set up. If you want to test your system,
try scanning it with [nmap][nmap].

Possible next steps:

* Install an Intrusion Detection System (IDS) like [psad][psad] to notify you
  whenever someone tries to detect intrusion attempts.
* Use [SELinux][selinux] to harden the kernel and give you access to some extra
  security utilities.
* Remove KDE/Gnome desktops. A desktop on a server is unnecessary, consumes
  resources, and provides a larger attack surface.
* Monitor all activity on the server using [psacct or acct][acct]. These
  applications runs in the background and keeps track of each users activity
  on your system as well as what resources are being consumed

And of course, make sure your server always has the latest security updates.


[ufw]: https://wiki.archlinux.org/index.php/Uncomplicated_Firewall
[psad]: https://www.digitalocean.com/community/tutorials/how-to-use-psad-to-detect-network-intrusion-attempts-on-an-ubuntu-vps
[nmap]: https://nmap.org/
[selinux]: https://en.wikipedia.org/wiki/Security-Enhanced_Linux
[acct]: http://www.tecmint.com/how-to-monitor-user-activity-with-psacct-or-acct-tools/
