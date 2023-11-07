# atproto-rs

A rust crate to connect to a bluesky PDS and utilise the Authenticated Transfer protocol

## Currently implemented
- com.atproto
    - com.atproto.server.createInviteCode (ATP.create_invite_code)
    - com.atproto.server.createAccount (ATP.create_account)
    - com.atproto.server.createSession (ATP.login)
    - com.atproto.repo.createRecord (ATP.post)

`Note: ATP.post is specifically for a bluesky post. Replies and reposts are not currently supported but planned.`

## Example Code
A quick example GUI powered by the crate

Run with:

```
cargo run --bin atproto-rs-example
```

If you are using the main bluesky use `https://bsky.social` as the provider otherwise use the URL to your own PDS. If you are able to use app passwords please do so for security.