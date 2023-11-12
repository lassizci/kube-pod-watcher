# kube-pod-watcher
Kubernetes pod watcher in Rust. Simply watches pods and dumps the struct as output

## build & run

Locally, against a remote Kubernetes cluster, using local kubeconfig and current context.
```shell
cargo build --release
./target/release/kube-pod-watcher
```
