# 💖 emojeez 💘

## [`💖 get a great heart`](https://emoji.bots.house/%F0%9F%92%96/?size=510&style=google)

## 👨‍🦯 examples

```html
<img src="https://emoji.bots.house/💖/?size=120&style=google" alt="heart">
```

## 🖼 documentation

### access emoji png

```http request
GET {uri}/{emoji|emoji_alias}/
```

Note: will return naively sized image/png as stored in emojipedia CDN

#### returns
- `200` success with image in response body
- `404` emoji was not found

### query params

Note: mixing up params does not matter (at least one of any is allowed).

#### `size`

```http request
GET {uri}/{emoji|emoji_alias}/?size={width}[:{height}]
```

Note: since most emoji pngs are M*M, giving only width is sure enough. 
Accessing bigger sizes of small images can be troublesome, but server will do its best. 
If we could not resize png, the server guarantees to return data at its original size.

##### returns
- `200` success with image in response body
- `404` emoji was not found

#### `style`

```http request
GET {uri}/{emoji|emoji_alias}/?style=[one_allowed_style]
```

##### 💅 supported emoji styles

    Mozilla
    Apple
    Google
    Twitter
    Samsung
    WhatsApp
    LG
    HTC
    OpenMoji
    Microsoft
    Facebook
    Messenger


Note: this query parameter is case-insensitive and defaults to `apple`

##### returns
- `200` success with image in response body
- `404` emoji was not found or style is unknown

### 🏓 ping

```http request
GET {uri}/ping/
```

- `200` with exact body `pong`
