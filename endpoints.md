# api: current endpoints:

## login

### /api/v1/login

| Method | Description |
| :-- | :-- |
| POST | create session |

note: how to ban a user immediatly / revoke token ?

## signup

### /api/v1/signup

| Method | Description |
| :-- | :-- |
| POST | register user |

todo: add email verification

## stigmarks

### /api/v1/stigmarks

| Method | Description |
| :-- | :-- |
| POST | create collection |
| GET | enumerate collections <br/> - from=:user_id |

### /api/v1/stigmarks/:collection_id

| Method | Description |
| :-- | :-- |
| GET | get collection | 
| DELETE | remove collection | 

### /api/v1/stigmarks/:collection_id/urls

| Method | Description |
| :-- | :-- |
| POST | add url to collection |
| GET | get urls of collection |

### /api/v1/stigmarks/:collection_id/urls/:url_id

| Method | Description |
| :-- | :-- |
| DELETE | remove url from collection |

### /api/v1/stigmarks/:collection_id/keywords

| Method | Description |
| :-- | :-- |
| POST | add keyword to collection |
| GET | get keywords of collection |

### /api/v1/stigmarks/:collection_id/keywords/:keyword_id

| Method | Description |
| :-- | :-- |
| DELETE | remove keywords from collection |

## search          

    note: this will search into all users except private ones I'm not following

    todo: search by synonyms, incomplete words, etc...

### /api/v1/search

| Method | Description | Parametes |
| :-- | :-- | :-- |
| GET | search | q=:keywords <br /> from=:user_id |

    todo: add stuff from IA team

## stigmers: the ones I follow

### /api/v1/stigmers        

| Method | Description | Note |
| :-- | :-- | :-- |
| POST | request subscription to stigmer | todo: notify stigmer and wait for authorization |
| GET | get stigmers | todo: subscription can be pending |

## followers: the ones who follow me

### /api/v1/followers

| Method | Description |
| :-- | :-- |
| GET | get followers |

### /api/v1/followers/:user_id

| Method | Description |
| :-- | :-- |
| PUT | authorize follower to follow (set authorized_at, remove forbidden_at) |
| DELETE | remove from followers (set forbidden_at, remove authorized_at) |

# todo: events

### /api/v1/events/like/:url_id

| Method | Description |
| :-- | :-- |
| POST | like url |
| DELETE | unlink url |

# todo: admin stuff
      
### /api/v1/logs ?

### /api/v1/stats ?