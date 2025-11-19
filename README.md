# Rusty streamer

```bash
openapi-generator-cli generate -g rust --additional-properties=library=reqwest,supportAsync=true,packageName=movies_client,supportMiddleware=true -i ../api-docs/openapi.json skip-validate-spec
```
https://openapi-generator.tech/docs/generators
openapi-generator-cli generate -g typescript-axios --additional-properties= -i ../bff/api-docs/openapi.json --skip-validate-spec -o generated
# Architecture overview
![Architecture](./Arch.png)

az storage cors add `
  --services b `
  --methods GET PUT POST DELETE OPTIONS `
  --origins "*" `
  --allowed-headers "*" `
  --exposed-headers "*" `
  --max-age 86400 `
  --connection-string "UseDevelopmentStorage=true"
