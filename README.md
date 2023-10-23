# Al Arkhabil server software

## API endpoints

### Invites

#### GET /api/v1/invite/new?token={new invite token}

### Accounts

#### POST /api/v1/account/new

**Post data:** Alarkhabil-ed25519-signed JSON

Payload:

```
{
    "uuid": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
    "name": "<name>",
    "invite": "<invite token string>"
}
```
