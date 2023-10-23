# Al Arkhabil server software

## API endpoints

### Invites v1

#### GET /api/v1/invite/new?token={new invite token}

**Response type:** JSON

Response:

```
{
    "invite": "<invite token string (base64)>"
}
```

### Accounts v1

#### POST /api/v1/account/new

**Post data:** Alarkhabil-ed25519-signed JSON

**Response type:** JSON

Payload:

```
{
    "uuid": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
    "name": "<name>",
    "invite": "<invite token string (base64)>"
}
```

Response example:

```
{
    "status": "ok"
}
```
