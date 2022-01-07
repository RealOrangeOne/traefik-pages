# traefik-pages

![CI](https://github.com/RealOrangeOne/traefik-pages/workflows/CI/badge.svg)

Website hosting server (think GitHub Pages) designed to deeply integrate with Traefik for routing and TLS termination.

**Work in progress**

## Usage

Create a directory of directories, where the name of each directory is the hostname of a site you want to serve, with its content inside.

```
/mnt/sites
├── example.com
│   └── index.html
└── othersite.example.com
    └── index.html
```

How the files get there is up to you. [Minio](https://min.io/), `rsync`, webdav, `ansible`, doesn't matter.

### How it works

`traefik-pages` integrates with Traefik via the [HTTP provider](https://doc.traefik.io/traefik/providers/http/). When Traefik hits the API, `traefik-pages` lists the directories containing sites to get the hostnames required, and returns a configuration of routers for Traefik to use. These routers have rules matching the hostnames from the directories, and services matching the one specified for `traefik-pages`. Traefik constantly polls `traefik-pages` for an updated configuration, so newly created sites wll be quickly picked up on.

## Installation

First, create a container for `traefik-pages`:

```yml
  traefik-pages:
    image: theorangeone/traefik-pages:latest
    volumes:
      - ./sites:/mnt/sites:ro
    environment:
      - SITES_ROOT=/mnt/sites
      - TRAEFIK_SERVICE=traefik-pages@docker
      - AUTH_PASSWORD=hunter2
    labels:
      - traefik.enable=true
```

This doesn't need to be in the same file as Traefik, but it does need to be accessible to Traefik using a fixed hostname and IP. If Traefik is running in host mode (as I do), you'll need to bind `traefik-pages` to an internal interface, and listen to that.

The labels enable traefik autoconfiguration and configure the service listener. Note that the service name should match `$TRAEFIK_SERVICE`. The second label can be omitted, but $TRAEFIK_SERVICE will need to match the automatically configured name of the service.

Next, you'll need to create a HTTP provider for Traefik, using the ports and password previously configured.

```yml
providers:
  ...
  http:
    endpoint:
      - "http://hunter2@127.0.0.1:5000/.traefik-pages/provider"
```

Here you can also configure the polling interval for `traefik-pages`.


Now, simply start Traefik and `traefik-pages`, and they should begin communicating and creating routers for your sites.

## Configuration

Configuration for `traefik-pages` is done entirely through environment variables:

- `$SITES_ROOT`: Directory where sites are stored (required).
- `$TRAEFIK_SERVICE`: Service name for `traefik-pages`, where traffic will be routed (required).
- `$AUTH_PASSWORD`: Basic auth username required for access to private URLs (`/.traefik-pages/*`) (required).

- `$DENY_PREFIXES`: Comma-separated list of URL prefixes to ignore (immediately return 404). Empty by default.
- `$LOG_INTERNAL`: Whether to log requests for internal URLs (default false).
- `$TRAEFIK_CERT_RESOLVER`: Traefik certificate resolver to use to provision TLS certificates (by default no certificates will be requested).
- `$PORT`: Port to listen on (default 5000).
- `$WORKERS`: Number of worker processes to handle requests (default 1).

## Performance

`traefik-pages` is written in Rust, and designed to be as fast as possible.

```
Requests per second:    6786.85 [#/sec] (mean)
Time per request:       14.734 [ms] (mean)
Time per request:       0.147 [ms] (mean, across all concurrent requests)
Transfer rate:          1471.37 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.1      0       2
Processing:     3   15   5.5     14      51
Waiting:        2   14   5.5     14      51
Total:          3   15   5.5     14      51

Percentage of the requests served within a certain time (ms)
  50%     14
  66%     16
  75%     17
  80%     18
  90%     21
  95%     25
  98%     30
  99%     35
 100%     51 (longest request)
```

These tests were run on a 2600X, with a single worker process.
