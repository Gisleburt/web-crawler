Web Crawler
===========

A simple web crawler that gets information about links at a given url. There are two variants, a cli application
and a web server.

CLI
---

### Usage:

```shell
$ crawl https://example.com
http://www.iana.org/domains/example - 1
```

Server
------

### Usage:

To use locally, start the server with:

```shell
$ PORT=8080 crawl-api
```

Then visit the server with a url encoded url of where you'd like to begin the crawl, eg

htt://localhost:8080/https%3A%2F%2Fexample.com

The server reads two environment variables, `PORT` which sets which port it should listen on, and `PRODUCTION` which
tells the server if it should listen to all incoming connections or not. Use `PRODUCTION=1` if you want the server to
be visible on your network.

Docker
------

### Usage

To start the api with docker, first build it, then start it and tell it which port to listen to:

```shell
$ docker build . -t crawler
$ docker run --rm -it -p 8080:80 crawler
```

Tests
-----

There are both doc tests and unit tests, `cargo test` will run them all

```shell
$ cargo test
```

Docs
----

There are some docs, but I would have liked to have done more:

```shell
$ cargo doc --no-deps --open
```

Notes:
------

I'm glad I started with cli as it let me get going very quickly. I've been experimenting with this multi-binary approach
lately and think it has a lot going for it. With more time I'd have liked to make the output a bit prettier.

Actix have change the way their framework works to prevent blocking on normal endpoints. If you need to block, which we
do to use Reqwest, we have to use Futures which I hadn't touched before this so it took some time to wrangle the types
the way Actix was looking for. Worth noting that Actix used the Futures crate rather than `std::futures`, so this may
change in the future.

There were a few other things I want to try but didn't get a chance to, such as using `Rayon` or `Reqwest/r#async` with
an `Arc<Mutex<UrlSummary>>` to see if we could speed up the crawling. I also would have liked to improve the way you
request a call from the API and if you were to make this accessible anywhere you'd probably want to heavily rate limit
it / restrict it as crawling takes a long time. 

Finally, if you want to run the apps from cargo, don't forget to tell it which bin, eg:

```shell
$ cargo run --bin crawl -- https://example.com
```
