# atproto-rs

A rust crate to connect to a bluesky PDS and utilise the Authenticated Transfer protocol

## Currently implemented
- com.atproto
    - com.atproto.server.createInviteCode (ATP.create_invite_code)
    - com.atproto.server.createAccount (ATP.create_account)
    - com.atproto.server.createSession (ATP.login)
    - com.atproto.repo.createRecord (ATP.post)

Note: ATP.post is specifically for a bluesky post. Replies and reposts are not currently supported.