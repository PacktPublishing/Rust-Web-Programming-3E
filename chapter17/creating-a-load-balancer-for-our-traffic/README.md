
## Preparing the build

Before we can run the application we need to build the image with the following command:

```bash
# for aarch chips
docker build . -f Dockerfile.aarch -t compute-unit

# for x86 chips
docker build . -f Dockerfile.x86 -t compute-unit
```


## Running the monolithic application

To run the monolithic application, you run the following command:

```bash
docker-compose -f docker-compose.yml -f ingress/build-config.yml up
```

## Running the microservices application

To run the microservices application, you run the following command:

```bash
docker-compose -f docker-compose.yml -f nanoservices/auth/build-config.yml -f nanoservices/to_do/build-config.yml -f ingress/frontend/build-config.yml up
```

## Create a user

To create a user, you can use the following command:

```bash
sh ingress/scripts/create_login.sh
```
