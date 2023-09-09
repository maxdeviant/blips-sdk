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
graphql-client generate --schema-path=schema.json --custom-scalars-module='crate::graphql::custom_scalars' --response-derives='Debug' --output-directory crates/blips/src/graphql/generated/ crates/blips/src/graphql/
```
