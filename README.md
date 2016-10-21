# rust-dockerclient

This package presents a client for the Docker remote API. It currently supports the Docker API up to version 1.23.

Note that docker's network API is only available in docker 1.8 and above, and only enabled in docker if DOCKER_EXPERIMENTAL is defined during the docker build process.

For more details, check the [remote API
documentation](https://docs.docker.com/engine/reference/api/docker_remote_api/).

## How it works?

```rust
extern crate docker;

use docker::Docker;

fn main() {
    let endpoint = "/var/run/docker.sock";
    let client = Docker::new(endpoint);
    let images = client.list_images();
}
```

Inspired by [go-dockerclient](https://github.com/fsouza/go-dockerclient)
