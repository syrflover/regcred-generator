# regcred generator

## usage

```sh
cargo run -- -u <username> -p <password> <host>
# or
cargo build --release
./target/release/regcred -u <username> -p <password> <host>
```

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: <secret-name>
  namespace: <namespace>
type: kubernetes.io/dockerconfigjson
data:
  .dockerconfigjson: <output of regcred command>
```
