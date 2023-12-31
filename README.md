# Al Arkhabil API server

Backend API server for Al Arkhabil, the independent thought publication platform.

* [Frontend code Github](https://github.com/metastable-void/alarkhabil-frontend)

## About this API

* Strings are always in UTF-8.
* Maximum post size in Markdown is 100kB.
* Maximum bio/channel description size in Markdown is 4kB.
* File uploads/user icons are not supported in the first release, but will be supported in future releases.
* API pagination is not supported first. Dangerous queries are limited to 1000 entries.
* Body text (including channel/author descriptions) strings are assumed to be in Markdown. Titles and names are not. Currently this server does not parse Markdown.

### Tokens

* __Invite making token__: hex-encoded string of random data. Can be used to create a new invite.
* __Admin token__: hex-encoded string of random data. Can be used for administrative actions (e.g. deletion of users, etc.).
* __Invite token__: base64-encoded string containing message signed by the server. Can be used for requesting a new account. Can be parsed freely to get the invited user's UUID.

## List of v1 endpoints

All endpoints are below `/api/v1/`.

Regex for ValidDnsToken(): `^[a-z0-9]+(-[a-z0-9]+)*$`

* __Account__ means the authenticating user's account.
* __self__ means the author object of the authenticating user.
* __Author__ is an author object.

Method | URL | Auth | Invariant | Input
-------|-----|------|-----------|------
GET | invite/new | `{invite making token}` | - | Query: `token`
POST | account/new | Self-signed by new public key | Public key does not exist yet on DB | Signed JSON data (POST)
POST | account/check_credentials | **Pubkey account auth** | NotDeleted(Account) | Signed JSON data (POST)
POST | account/change_credentials | **Pubkey account auth** (Signed by old public key) | NotDeleted(Account) && Valid signature by new public key included | Signed JSON data (POST)
POST | account/delete | **Pubkey account auth** | NotDeleted(Account) | Signed JSON data (POST)
POST | admin/meta/update | `{admin token}` | ValidDnsToken(`page_name`) | Query: `token`; Plain JSON data (POST)
POST | admin/meta/delete | `{admin token}` | MetaPageExists(`page_name`) | Query: `token`, `page_name`; Empty POST data
POST | admin/author/delete | `{admin token}` | AuthorExists(`uuid`) | Query: `token`, `uuid`; Empty POST data
POST | admin/channel/delete | `{admin token}` | ChannelExists(`uuid`) | Query: `token`, `uuid`; Empty POST data
POST | admin/post/delete | `{admin token}` | PostExists(`uuid`) | Query: `token`, `uuid`; Empty POST data
POST | self/update | **Pubkey account auth** | NotDeleted(Account) | Signed JSON data (POST)
POST | channel/new | **Pubkey account auth** | NotDeleted(Account) && !ChannelExists(`handle`) && ValidDnsToken(`handle`) | Signed JSON data (POST)
POST | channel/update | **Pubkey account auth** | NotDeleted(Account) && NotDeleted(Channel) && Owns(Channel) && NoConflict(`handle`) && ValidDnsToken(`handle`) | Signed JSON data (POST)
POST | channel/delete | **Pubkey account auth** | NotDeleted(Account) && NotDeleted(Channel) && Owns(Channel) | Signed JSON data (POST)
POST | channel/add_author | **Pubkey account auth** | NotDeleted(Account) && NotDelete(Channel) && Owns (Channel) && NotDeleted(Author) && Account != Author | Signed JSON data (POST)
POST | channel/remove_author | **Pubkey account auth** | NotDeleted(Account) && NotDelete(Channel) && Owns (Channel) && NotDeleted(Author) && Account != Author | Signed JSON data (POST)
POST | post/new | **Pubkey account auth** | NotDeleted(Account) && NotDeleted(Channel) && Owns(Channel) | Signed JSON data (POST)
POST | post/update | **Pubkey account auth** | NotDeleted(Account) && NotDeleted(Channel) && NotDeleted(Post) && Owns(Channel) | Signed JSON data (POST)
POST | post/delete | **Pubkey account auth** | NotDeleted(Account) && NotDeleted(Channel) && NotDeleted(Post) && Owns(Channel) | Signed JSON data (POST)
GET | meta/info | - | MetaPageExists(`page_name`) | Query: `page_name`
GET | meta/list | - | - | -
GET | author/info | - | NotDeleted(Author) | Query: `uuid`
GET | author/list | - | NotDeleted(Author) | -
GET | author/channels | - | NotDeleted(Author) && NotDeleted(Channel) | Query: `uuid`
GET | author/posts | - | NotDeleted(Author) && NotDeleted(Channel) && NotDeleted(Post) && NotDeleted(Revision) | Query: `uuid`
GET | channel/info | - | NotDeleted(Channel) | Query: `uuid` or `handle`
GET | channel/list | - | NotDeleted(Channel) | -
GET | channel/authors | - | NotDeleted(Channel) && NotDeleted(Author) | Query: `uuid`
GET | channel/posts | - | NotDeleted(Channel) && NotDeleted(Post) | Query: `uuid`
GET | post/info | - | NotDeleted(Post) && NotDeleted(Channel) [ && HasUndeleted(Revision) ] | Query: `uuid`
GET | post/list | - | NotDeleted(Post) && NotDeleted(Channel) [ && HasUndeleted(Revision) ] | -
GET | tag/list | - | NotDeleted(Post) && NotDeleted(Channel) && HasUndeleted(Revision) | -
GET | tag/posts | - | NotDeleted(Post) && NotDeleted(Channel) && HasUndeleted(Revision) | -

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

### POST /api/v1/account/check_credentials

Basically a no-op authenticated request. Returns an error if authentication fails.

**Post data:** Alarkhabil-ed25519-signed JSON

**Response type:** JSON

Will return **400 Bad Request** for invalid requests.

Payload:

```
{
    "command": "account_check_credentials"
}
```

Response example:

```
HTTP/1.1 200
{
    "status": "ok"
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

### POST /api/v1/admin/meta/update

Creates or updates a meta page.

**Query format:** `?token={admin token}`

**Post data:** JSON

**Response type:** JSON

Will return **400 Bad Request** for invalid requests.

Request:

```
{
    "page_name": "<name of meta page (part of url)>",
    "title": "<title of meta page>",
    "text": "<markdown text of meta page>"
}
```

Response:

```
HTTP/1.1 200
{
    "status": "ok"
}
```

### POST /api/v1/admin/meta/delete

Deletes a meta page. This is irreversible.

**Query format:** `?token={admin token}&page_name={page name}`

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
    "created_date": <registration date in seconds since UNIX epoch (integer)>
    "description_text": "<description markdown>"
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
    "created_date": <seconds since UNIX epoch (integer)>
    "lang": "<channel's language code>",
    "description_text": "<description markdown>"
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
    "created_date": <seconds since UNIX epoch (integer)>
    "lang": "<channel's language code>",
    "description_text": "<description markdown>"
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

### POST /api/v1/channel/add_author

You cannot add yourself as an author.

**Post data:** Alarkhabil-ed25519-signed JSON

**Response type:** JSON

Will return **400 Bad Request** for invalid requests.

Payload:

```
{
    "command": "channel_add_author",
    "uuid": "<channel's uuid>",
    "author_uuid": "<author's uuid>"
}
```

Response example:

```
HTTP/1.1 200
{
    "status": "ok"
}
```

### POST /api/v1/channel/remove_author

You cannot remove yourself from a channel.

**Post data:** Alarkhabil-ed25519-signed JSON

**Response type:** JSON

Will return **400 Bad Request** for invalid requests.

Payload:

```
{
    "command": "channel_remove_author",
    "uuid": "<channel's uuid>",
    "author_uuid": "<author's uuid>"
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
    "revision_date": <revision date in seconds since UNIX epoch>,
    "title": "<title>",
    "author": {
        "uuid": "<author's uuid>",
        "name": "<author's name>"
    },
    "revision_text": "<revision text>",
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
    "uuid": "<post's uuid>",
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

### GET /api/v1/meta/info

**Query format:** `?page_name={meta page name}`

**Response type:** JSON

Response (post found):

```
HTTP/1.1 200
{
    "page_name": "<name of meta page>",
    "updated_date": <revision date in seconds since UNIX epoch>,
    "title": "<title>",
    "text": "<page markdown text>"
}
```

Response (post not found):

```
HTTP/1.1 404
{
    "status": "not found"
}
```

### GET /api/v1/meta/list

**Query format:** (none) - TODO: allow paging

**Response type:** JSON

Response:

```
HTTP/1.1 200
[
    {
        "page_name": "<name of meta page>",
        "updated_date": <revision date in seconds since UNIX epoch>,
        "title": "<title>"
    },
    ...
]
```

### GET /api/v1/author/info

**Query format:** `?uuid={author uuid}`

**Response type:** JSON

Response (author found):

```
HTTP/1.1 200
{
    "uuid": "<author's uuid>",
    "name": "<author's name>",
    "created_date": <registration date in seconds since UNIX epoch (integer)>
    "description_text": "<description markdown>"
}
```

Response (author not found or deleted):

```
HTTP/1.1 404
{
    "status": "not found"
}
```

### GET /api/v1/author/list

The results are ordered with the newest registration first.

**Query format:** (none) - TODO: allow paging

**Response type:** JSON

Response:

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
        "name": "<channel's name>",
        "lang": "<channel's language code>"
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
            "name": "<channel's name>",
            "lang": "<channel's language code>"
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
    "created_date": <seconds since UNIX epoch (integer)>
    "lang": "<channel's language code>",
    "description_text": "<description markdown>"
}
```

Response (channel not found or deleted):

```
HTTP/1.1 404
{
    "status": "not found"
}
```

### GET /api/v1/channel/list

The results are ordered with the newest channel first.

**Query format:** (none) - TODO: allow paging

**Response type:** JSON

Response:

```
HTTP/1.1 200
[
    {
        "uuid": "<channel's uuid>",
        "handle": "<channel's handle>",
        "name": "<channel's name>",
        "lang": "<channel's language code>"
    },
    ...
]
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

### GET /api/v1/post/list

The results are ordered with the newest post first.

**Query format:** (none) - TODO: allow paging

**Response type:** JSON

Response:

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
        },
        "channel": {
            "uuid": "<channel's uuid>",
            "handle": "<channel's handle>",
            "name": "<channel's name>",
            "lang": "<channel's language code>"
        }
    },
    ...
]
```

### GET /api/v1/tag/list

**Query format:** (none) - TODO: allow paging

**Response type:** JSON

Response:

```
HTTP/1.1 200
[
    {
        "tag_name": "<tag name>",
        "page_count": <page count>
    },
    ...
]
```

### GET /api/v1/tag/posts

The results are ordered with the newest post first.

**Query format:** `?tag_name={tag name}` - TODO: allow paging

**Response type:** JSON

Response:

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
        },
        "channel": {
            "uuid": "<channel's uuid>",
            "handle": "<channel's handle>",
            "name": "<channel's name>",
            "lang": "<channel's language code>"
        }
    },
    ...
]
```

## Build

```
cargo build
```

## Configuration

```
cp ./example.env ./.env
# edit ./.env
```

## License

Licensed under the Apache 2.0 license.

### Authors

- [@metastable-void](https://github.com/metastable-void)
