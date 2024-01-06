# dozy-host

## Push to DockerHub

```sh
DOCKER_BUILDKIT=1 docker build -t crystal27/dozy-host:<tag> .
docker push crystal27/dozy-host:<tag>
```

If the `docker push` fails with:

> denied: requested access to the resource is denied

It is likely that you are not logged into DockerHub. Run: `docker login` to fix.