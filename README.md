# Rusty streamer

```bash
openapi-generator-cli generate -g rust --additional-properties=library=reqwest,supportAsync=true -i ../../api-docs/openapi.json --skip-validate-spec
```
https://openapi-generator.tech/docs/generators

# Architecture overview
![Architecture](./Arch.png)