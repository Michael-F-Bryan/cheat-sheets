Common Docker Images
====================

Here are several examples of Docker images and how to use them.


Postgres
--------

Full documentation [here][1].

To create a temporary Postgres container:

    docker run -d \
        -e POSTGRES_PASSWORD=mysecretpassword \
        -p 5432:5432 \
        postgres

This will create a Postgres database listening on `localhost:5432`, with the
username `postgres` and the password `mysecretpassword`. The database name also
defaults to `postgres`.

The following environment variables allow you to change various aspects of the
container:

- `POSTGRES_USER` - Change the default user
- `POSTGRES_PASSWORD` - Alter the user's password
- `PGDATA` - Define another location for the database files (default:
    /var/lib/postgresql/data)
- `POSTGRES_DB` - Define a different name for the default database

For persistence, you will often want to add `-v
/path/to/data:/var/lib/postgresql/data` to the command to let the container
write to a directory on the host OS.


Redis
-----

Full documentation [here][2].

Starting the image is fairly easy:

    docker run -d -p 6379:6379 redis

If you want to access the data directory, or share it as a volume for whatever
reason, the persistent data is stored in the image's `/data` directory.


Static Files With Nginx
-----------------------

Full documentation [here][3].

This uses a 3rd party docker image because the official one is too general. It
will serve anything in `/var/www` and EXPOSEs both port 80 and 443.

    docker run -d -p 80:80 -v /tmp/share:/var/www:ro kyma/docker-nginx


Celery Worker
-------------

Full documentation [here][4].

To spin up a celery worker and attach it to a running redis instance, do:

    docker run --link some-redis:redis --name some-celery -e CELERY_BROKER_URL=redis://redis -d celery

If you are wanting to check the status of the cluster, you can temporarily
attach a new celery worker and call `celery status`.

    docker run --link some-redis:redis -e CELERY_BROKER_URL=redis://redis --rm celery celery status



[1]: https://hub.docker.com/_/postgres/
[2]: https://hub.docker.com/_/redis/
[3]: https://hub.docker.com/r/kyma/docker-nginx/
[4]: https://hub.docker.com/_/celery/
