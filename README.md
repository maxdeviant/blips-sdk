# Blips

## Development

### Update GraphQL Schema

```sh
graphql-client introspect-schema \
    --header 'Cookie: <COOKIE_VALUE>' \
    --header 'X-Csrf-Token: <CSRF_TOKEN>' \
    https://blips.app/query --output schema.json

```

### Run GraphQL Codegen

```sh
cargo run -p blips_codegen
```
