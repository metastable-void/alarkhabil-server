# Al Arkhabil server software

## API endpoints

### Invites v1

#### GET /api/v1/invite/new?token={new invite token}

The administrator would have access to their *new invite token*.

The administrator uses the *new invite token* to make a request of this type, and they will get an invite token in base64, which they can tell someone.

**Response type:** JSON

Response:

```
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
{
    "status": "ok",
    "uuid": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
}
```
