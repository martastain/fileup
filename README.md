fileup
======

fileup is a dead simple file upload server. Just run it and it will start listening on port 6547.
The port can be changed by setting the environment variable `FILEUP_PORT` or  using a cli argument.

```
Options:
      --server-port <SERVER_PORT>  [env: FILEUP_PORT=] [default: 6547]
      --target-dir <TARGET_DIR>    [env: FILEUP_DIR=] [default: uploads]
      --log-level <LOG_LEVEL>      [env: HTTP2AMCP_LOG_LEVEL=] [default: info]
  -h, --help                       Print help
  -V, --version                    Print version
```


## Usage

Then just use:

```bash
curl -T /my/local/file.mov -X POST http://localhost:6547/upload/targetfile.mov
```

And the file will be uploaded to the server in the `uploads` directory (that will be created automagically if it does not exists)


## Note

This was created in order to solve one particular problem. It is not meant to be used by general audience.
