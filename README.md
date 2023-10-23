# Al Arkhabil server software

## API documentation

* Strings are always in UTF-8.
* Maximum post size in Markdown is 100kB.
* Maximum bio/channel description size in Markdown is 4kB.
* File uploads/user icons are not supported in the first release, but will be supported in future releases.

### Admin endpoints v1

#### GET /api/v1/invite/new

The administrator would have access to their *new invite token*.

The administrator uses the *new invite token* to make a request of this type, and they will get an invite token in base64, which they can tell someone.

**Query format:** `?token={new invite token}`

**Response type:** JSON

Will return **400 Bad Request** for invalid requests.

Response:

```
HTTP/1.1 200
{
    "invite": "<invite token string (base64)>"
}
```

#### POST /api/v1/admin/author/delete

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

#### POST /api/v1/admin/channel/delete

**Query format:** `?token={admin token}&uuid={channel's uuid}`

**Query format:** `?token={admin token}&handle={channel's handle}`

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

#### POST /api/v1/admin/post/delete

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

### Accounts v1

#### POST /api/v1/account/new

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

### Public endpoints v1

#### GET /api/v1/author/info

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

#### GET /api/v1/author/channels

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

#### GET /api/v1/author/posts

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

#### GET /api/v1/channel/info

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

#### GET /api/v1/channel/authors

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

#### GET /api/v1/channel/posts

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

#### GET /api/v1/post/info

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
        "name": "<channel's name>"
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

### Authors' endpoints v1


