# mongodb

## About

[**📚 Read this template tutorial! 📚**](https://roche-rs.org/tutorials/mongodb.html)

This template is designed roche and is used for compiling Rust libraries into docker and 
publishing the resulting package as knative service.


dev-mongodb - [![Docker Repository on Quay](https://quay.io/repository/roche/dev-mongodb/status "Docker Repository on Quay")](https://quay.io/repository/roche/dev-mongodb)

mongodb - [![Docker Repository on Quay](https://quay.io/repository/roche/mongodb/status "Docker Repository on Quay")](https://quay.io/repository/roche/mongodb)

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
roche build
```

### 🔬 Testing the library

```
docker run -p 27017:27017 -v ~/data:/data mongo:4.2
roche test
```

