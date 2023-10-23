# Al Arkhabil server software

## API documentation

* Strings are always in UTF-8.
* Maximum post size in Markdown is 100kB.
* Maximum bio/channel description size in Markdown is 4kB.
* File uploads/user icons are not supported in the first release, but will be supported in future releases.

### Invites v1

#### GET /api/v1/invite/new

The administrator would have access to their *new invite token*.

The administrator uses the *new invite token* to make a request of this type, and they will get an invite token in base64, which they can tell someone.

**Query format:** `?token={new invite token}`
**Response type:** JSON

Response:

```
HTTP/1.1 200
{
    "invite": "<invite token string (base64)>"
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

