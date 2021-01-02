# mongodb

## About

[**📚 Read this template tutorial! 📚**](https://roche-rs.org/tutorials/project.html)

This template is designed roche and is used for compiling Rust libraries into docker and 
publishing the resulting package as knative service.

## 🚴 Usage

### Use 🐑 `roche init mongodb` to Clone this Template

Roche uses the excellent cargo-generate under the hood.
[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
roche init mongodb --name my-project
cd my-project
```

### 🛠️ Build with `roche build`

```
docker login
roche build -b quay.io/roche/dev-mongodb:1.0.0
```

### 🔬 Testing the library

```
roche test -l quay.io/roche/dev-mongodb:1.0.0
```

