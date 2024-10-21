Why separate deploy branch and build locally?

Because installing `dioxus-cli` takes a long time.
If the build is done using `github actions`, it usually took about 10 minutes to install `dioxus-cli`.

-------------------------------------------

# Development

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

- Open the browser to http://localhost:8080