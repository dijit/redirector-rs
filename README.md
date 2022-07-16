# Redirector

a redirector written in rust intended for *permanent* human-readable redirects.

The idea was semi-inspired by the book [Software Engineering at Google].

[This chapter](https://abseil.io/resources/swe-book/html/ch03.html#ch01fn44) describes `go/` links which
are part of knowledge sharing, canonical URLs, and are part of the inspiration for this topic.

The URL Redictor can be compiled with a static config-file backend or a database.

The database backend supports user submissions through 404 pages.

## Features

| feature     | excludes    | default |
|-------------|-------------|---------|
| config_file | database    | yes     |
| database    | config_file | no      |

## Adding URLs (Config_File)

[urls.toml](./urls.toml) contains a `[urls]` keyspace which contains key/value pairs of shortcodes and the URL.

For a real-life example, a config like:
```toml
[urls]
handbook = "https://mycompany.atlassian.net/wiki/spaces/DOCTEAM/pages/2565079041/Handbook"
```
would redirect `go/handbook` to `https://mycompany.atlassian.net/wiki/spaces/DOCTEAM/pages/2565079041/Handbook`
with a 308 (permanent) redirect. Assuming of course that this program was resolvable as `go`.

## Improvements

Future improvements could be a database and an API which can push updates to the dataset.

This will slow the process down considerably, but allows for much more flexibility, as the current
update mechanism is a merge-request.

[Software Engineering at Google]: https://www.oreilly.com/library/view/software-engineering-at/9781492082781/