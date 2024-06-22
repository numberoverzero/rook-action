# A very fast github action to send signed post requests with bodies

* Runs in ~0sec.  Uses a static rust binary to sign your POST body and send it to your endpoint.
* 900kB docker image prebuilt `FROM scratch`
* Simple, readable source (~200 lines)

The signature is a simple [HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC) with the same header structure as github's signed events.  The value is passed in the `"x-rook-signature-256"` header, prepended with `"sha256="`.

If you just want a fast github action to post some data somewhere, and don't care about authenticating the payload (yikes!) this should still outperform any node action or a docker image that runs `apt-install curl`.

TLS support planned.

# Sample configuration

```yml
# your-repository/.github/workflows/main.yml
on: ...

jobs:
  my_important_job:
    runs-on: ..
    name: ...
    env:
      STATUS_ENDPOINT: http://some-slack-bot-probably.yourcompany.com:9000
    steps:
      - name: ..
        uses: ..
      - name: send status
        id: ..
        uses: numberoverzero/rook-action@v2
        with:
          endpoint: ${{ env.STATUS_ENDPOINT }}
          secret: ${{ secrets.ROOK_SHARED_SECRET }}
          body: "github run ${{ github.RUN_ID }} has completed"
```

## Curl equivalent

For the request body `"hello, world"` and the secret `"hunter2"` the signature (hmac sha256) is:

```
HMAC-SHA256("hunter2", "hello, world")
  = b157643c98205db6da3655511665a993ba5dc34d056233f3319622f5a32f704b
```

Therefore the following curl request and sample config are equivalent:

```sh
curl -X POST \
  http://your-endpoint.com:9000/some/path \
  -H "x-rook-signature-256: sha256=b157643c98205db6da3655511665a993ba5dc34d056233f3319622f5a32f704b" \
  -d "hello, world"
```

```yml
    steps:
      - name: basically-just-curl
        uses: numberoverzero/rook-action@v2
        with:
          endpoint: "http://your-endpoint.com:9000/some/path"
          secret: "hunter2"
          body: "hello, world"
```

# I don't have a thing that listens for webhooks yet

Set one up in 5 minutes with [rook](https://github.com/numberoverzero/rook).  7 lines in a config file, a 200KB binary, and you'll be processing authenticated webhooks.
