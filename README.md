# Al Arkhabil API server

API (backend) server for the independent thought publication platform.

## About this API

* Strings are always in UTF-8.
* Maximum post size in Markdown is 100kB.
* Maximum bio/channel description size in Markdown is 4kB.
* File uploads/user icons are not supported in the first release, but will be supported in future releases.
* API pagination is not supported first. Dangerous queries are limited to 1000 entries.

### Tokens

* __Invite making token__: hex-encoded string of random data. Can be used to create a new invite.
* __Admin token__: hex-encoded string of random data. Can be used for administrative actions (e.g. deletion of users, etc.).
* __Invite token__: base64-encoded string containing message signed by the server. Can be used for requesting a new account. Can be parsed freely to get the invited user's UUID.

## Invites v1

### GET /api/v1/invite/new

The administrator would have access to their *invite making token*.

The administrator uses the *invite making token* to make a request of this type, and they will get an invite token in base64, which they can tell someone.

**Note:** This endpoint uses GET method because it does not change the state on the server (in the first design).

**Query format:** `?token={invite making token}`

**Response type:** JSON

Will return **400 Bad Request** for invalid requests.

Response:

```
HTTP/1.1 200
{
    "status": "ok",
    "invite": "<invite token string (base64)>"
}
```

## Accounts v1

### POST /api/v1/account/new

The user who wants to create an account, creates an ed25519 key pair and signs the folowing payload with the private key.

The payload contains the new account's name and an invite token from `GET /api/v1/invite/new`.
The signed message contains the user's ed25519 public key.

The response will contain the new account's UUID.

**Post data:** Alarkhabil-ed25519-signed JSON

**Response type:** JSON

Will return **400 Bad Request** for invalid requests.

Payload:

```
{
    "command": "account_new",
    "name": "<name>",
    "invite": "<invite token string (base64)>"
}
```

Response example:

```
HTTP/1.1 200
{
    "status": "ok",
    "uuid": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
}
```

### POST /api/v1/account/change_credentials

**Post data:** Alarkhabil-ed25519-signed JSON

**Response type:** JSON

Will return **400 Bad Request** for invalid requests.

Payload:

```
{
    "command": "account_change_credentials",
    "new_algo": "<new public key's algorithm>",
    "new_public_key": "<base64-encoded new public key>",
    "signature": "<base64-encoded signature for old public key binary data by new public key>"
}
```

Response example:

```
HTTP/1.1 200
{
    "status": "ok"
}
```

### POST /api/v1/account/delete

**TODO: Is this really needed?**

**Post data:** Alarkhabil-ed25519-signed JSON

**Response type:** JSON

Will return **400 Bad Request** for invalid requests.

Payload:

```
{
    "command": "account_delete"
}
```

Response example:

```
HTTP/1.1 200
{
    "status": "ok"
}
```

## Admin v1

### POST /api/v1/admin/author/delete

**Query format:** `?token={admin token}&uuid={author's uuid}`

**Post data:** none

**Response type:** JSON

Will return **400 Bad Request** for invalid requests.

Response:

```
HTTP/1.1 200
{
    "status": "ok"
}
```

### POST /api/v1/admin/channel/delete

**Query format:** `?token={admin token}&uuid={channel's uuid}`

**Post data:** none

**Response type:** JSON

Will return **400 Bad Request** for invalid requests.

Response:

```
HTTP/1.1 200
{
    "status": "ok"
}
```

### POST /api/v1/admin/post/delete

**Query format:** `?token={admin token}&uuid={post's uuid}`

**Post data:** none

**Response type:** JSON

Will return **400 Bad Request** for invalid requests.

Response:

```
HTTP/1.1 200
{
    "status": "ok"
}
```

## Authors' endpoints v1

### POST /api/v1/self/update

**Post data:** Alarkhabil-ed25519-signed JSON

**Response type:** JSON

Will return **400 Bad Request** for invalid requests.

Payload:

```
{
    "command": "self_update",
    "name": "<new name>",
    "description_text": "<new description markdown>"
}
```

Response example (same as `/api/v1/author/info`):

```
HTTP/1.1 200
{
    "uuid": "<author's uuid>",
    "name": "<author's name>",
    "created_at": <registration date in seconds since UNIX epoch (integer)>
    "description_text": "<description markdown>",
    "description_html": "<description html>"
}
```

### POST /api/v1/channel/new

**Post data:** Alarkhabil-ed25519-signed JSON

**Response type:** JSON

Will return **400 Bad Request** for invalid requests.

Payload:

```
{
    "command": "channel_new",
    "handle": "<channel's handle>",
    "name": "<channel's name>",
    "lang": "<channel's language code>"
}
```

Response example (same as `/api/v1/channel/info`):

```
HTTP/1.1 200
{
    "uuid": "<channel's uuid>",
    "handle": "<channel's handle>",
    "name": "<channel name>",
    "created_at": <seconds since UNIX epoch (integer)>
    "lang": "<channel's language code>",
    "description_text": "<description markdown>",
    "description_html": "<description html>"
}
```

### POST /api/v1/channel/update

**Post data:** Alarkhabil-ed25519-signed JSON

**Response type:** JSON

Will return **400 Bad Request** for invalid requests.

Payload:

```
{
    "command": "channel_update",
    "uuid": "<channel's uuid>",
    "handle": "<channel's new handle>",
    "name": "<channel's new name>",
    "lang": "<channel's new language code>",
    "description_text": "<new description markdown>"
}
```

Response example (same as `/api/v1/channel/info`):

```
HTTP/1.1 200
{
    "uuid": "<channel's uuid>",
    "handle": "<channel's handle>",
    "name": "<channel name>",
    "created_at": <seconds since UNIX epoch (integer)>
    "lang": "<channel's language code>",
    "description_text": "<description markdown>",
    "description_html": "<description html>"
}
```

### POST /api/v1/channel/delete

**Post data:** Alarkhabil-ed25519-signed JSON

**Response type:** JSON

Will return **400 Bad Request** for invalid requests.

Payload:

```
{
    "command": "channel_delete",
    "uuid": "<channel's uuid>"
}
```

Response example:

```
HTTP/1.1 200
{
    "status": "ok"
}
```

### POST /api/v1/post/new

**Post data:** Alarkhabil-ed25519-signed JSON

**Response type:** JSON

Will return **400 Bad Request** for invalid requests.

Payload:

```
{
    "command": "post_new",
    "channel_uuid": "<channel's uuid>",
    "title": "<post title>",
    "text": "<post markdown text>",
    "tags": [
        "<tag>",
        ...
    ]
}
```

Response example (same as `/api/v1/post/info`):

```
HTTP/1.1 200
{
    "post_uuid": "<posts's uuid>",
    "channel": {
        "uuid": "<channel's uuid>",
        "handle": "<channel's handle>",
        "name": "<channel's name>",
        "lang": "<channel's language code>"
    },
    "revision_uuid": "<revision's uuid>",
    "revision_date": "<revision date in seconds since UNIX epoch>",
    "title": "<title>",
    "author": {
        "uuid": "<author's uuid>",
        "name": "<author's name>"
    },
    "revision_text": "<revision text>",
    "revision_html": "<revision html>",
    "tags": [
        "<tag>",
        ...
    ]
}
```

### POST /api/v1/post/update

**Post data:** Alarkhabil-ed25519-signed JSON

**Response type:** JSON

Will return **400 Bad Request** for invalid requests.

Payload:

```
{
    "command": "post_update",
    "uuid": "<channel's uuid>",
    "title": "<new post title>",
    "text": "<new post markdown text>",
    "tags": [
        "<tag>",
        ...
    ]
}
```

Response example (same as `/api/v1/post/info`):

```
HTTP/1.1 200
{
    "post_uuid": "<posts's uuid>",
    "channel": {
        "uuid": "<channel's uuid>",
        "handle": "<channel's handle>",
        "name": "<channel's name>",
        "lang": "<channel's language code>"
    },
    "revision_uuid": "<revision's uuid>",
    "revision_date": "<revision date in seconds since UNIX epoch>",
    "title": "<title>",
    "author": {
        "uuid": "<author's uuid>",
        "name": "<author's name>"
    },
    "revision_text": "<revision text>",
    "revision_html": "<revision html>",
    "tags": [
        "<tag>",
        ...
    ]
}
```

### POST /api/v1/post/delete

**Post data:** Alarkhabil-ed25519-signed JSON

**Response type:** JSON

Will return **400 Bad Request** for invalid requests.

Payload:

```
{
    "command": "post_delete",
    "uuid": "<post's uuid>"
}
```

Response example:

```
HTTP/1.1 200
{
    "status": "ok"
}
```

## Public endpoints v1

### GET /api/v1/author/info

**Query format:** `?uuid={author uuid}`

**Response type:** JSON

Response (author found):

```
HTTP/1.1 200
{
    "uuid": "<author's uuid>",
    "name": "<author's name>",
    "created_at": <registration date in seconds since UNIX epoch (integer)>
    "description_text": "<description markdown>",
    "description_html": "<description html>"
}
```

Response (author not found or deleted):

```
HTTP/1.1 404
{
    "status": "not found"
}
```

### GET /api/v1/author/channels

**Query format:** `?uuid={author uuid}`

**Response type:** JSON

Empty array will be returned if no channels are found.

Response (author found):

```
HTTP/1.1 200
[
    {
        "uuid": "<channel's uuid>",
        "handle": "<channel's handle>",
        "name": "<channel's name>"
    },
    ...
]
```

Response (author not found or deleted):

```
HTTP/1.1 404
{
    "status": "not found"
}
```

### GET /api/v1/author/posts

**Query format:** `?uuid={author uuid}`

**Response type:** JSON

Empty array will be returned if no posts are found.

Response (author found):

```
HTTP/1.1 200
[
    {
        "post_uuid": "<posts's uuid>",
        "revision_uuid": "<revision's uuid>",
        "revision_date": "<revision date in seconds since UNIX epoch>",
        "title": "<title>",
        "channel": {
            "uuid": "<channel's uuid>",
            "handle": "<channel's handle>",
            "name": "<channel's name>"
        }
    },
    ...
]
```

Response (author not found or deleted):

```
HTTP/1.1 404
{
    "status": "not found"
}
```

### GET /api/v1/channel/info

**Query format:** `?uuid={channel uuid}`

**Query format:** `?handle={channel handle}`

**Response type:** JSON

Response (channel found):

```
HTTP/1.1 200
{
    "uuid": "<channel's uuid>",
    "handle": "<channel's handle>",
    "name": "<channel name>",
    "created_at": <seconds since UNIX epoch (integer)>
    "lang": "<channel's language code>",
    "description_text": "<description markdown>",
    "description_html": "<description html>"
}
```

Response (channel not found or deleted):

```
HTTP/1.1 404
{
    "status": "not found"
}
```

### GET /api/v1/channel/authors

**Query format:** `?uuid={channel uuid}`

**Response type:** JSON

Response (channel found):

```
HTTP/1.1 200
[
    {
        "uuid": "<author's uuid>",
        "name": "<author's name>"
    },
    ...
]
```

Response (channel not found or deleted):

```
HTTP/1.1 404
{
    "status": "not found"
}
```

### GET /api/v1/channel/posts

**Query format:** `?uuid={channel uuid}`

**Response type:** JSON

Response (channel found):

Empty array will be returned if no posts are found.

```
HTTP/1.1 200
[
    {
        "post_uuid": "<posts's uuid>",
        "revision_uuid": "<revision's uuid>",
        "revision_date": "<revision date in seconds since UNIX epoch>",
        "title": "<title>",
        "author": {
            "uuid": "<author's uuid>",
            "name": "<author's name>"
        }
    },
    ...
]
```

Response (channel not found or deleted):

```
HTTP/1.1 404
{
    "status": "not found"
}
```

### GET /api/v1/post/info

**Query format:** `?uuid={post uuid}`

**Response type:** JSON

Response (post found):

```
HTTP/1.1 200
{
    "post_uuid": "<posts's uuid>",
    "channel": {
        "uuid": "<channel's uuid>",
        "handle": "<channel's handle>",
        "name": "<channel's name>",
        "lang": "<channel's language code>"
    },
    "revision_uuid": "<revision's uuid>",
    "revision_date": "<revision date in seconds since UNIX epoch>",
    "title": "<title>",
    "author": {
        "uuid": "<author's uuid>",
        "name": "<author's name>"
    },
    "revision_text": "<revision text>",
    "revision_html": "<revision html>",
    "tags": [
        "<tag>",
        ...
    ]
}
```

Response (post not found or deleted):

```
HTTP/1.1 404
{
    "status": "not found"
}
```
