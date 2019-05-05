# Kubernetes Webhook Authentication

## Introduction

## Configuration

### Example

```yaml
inbound_listener:
  address: 127.0.0.1:9000

database:
  name: heimdallr_dev
  host: localhost
  username: make_it_so_number_one
  password: super_secret_password_here

```

## Testing

```shell
http POST http://127.0.0.1:9000/api/authenticate kind=TokenReview apiVersion=authentication.k8s.io/v1beta1 spec:='{"token":"kitty"}'
```
