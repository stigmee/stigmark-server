# api: current endpoints:

## login

### /api/v1/login
| POST | create session |

note: how to ban a user immediatly / revoke token ?

## signup

### /api/v1/signup
|POST   |register user      |

todo: add email verification

## stigmarks

### /api/v1/stigmarks
| POST   |create collection      |
| GET    |enumerate collections  |
| ---    |from=:user_id          |

### /api/v1/stigmarks/:collection_id
          GET         -> get collection
          DELETE      -> remove collection
### /api/v1/stigmarks/:collection_id/urls
          POST        -> add url to collection
          GET         -> get urls of collection
### /api/v1/stigmarks/:collection_id/urls/:url_id
          DELETE      -> remove url from collection
### /api/v1/stigmarks/:collection_id/keywords
          POST        -> add keyword to collection
          GET         -> get keywords of collection
### /api/v1/stigmarks/:collection_id/keywords/:keyword_id
          DELETE      -> remove keywords from collection

## search          

      # note: this will search into all users except private ones I'm not following
      # todo: search by synonyms, incomplete words, etc...
### /api/v1/search
          GET         -> search
              q=:keywords
              from=:user_id
          # todo: add stuff from IA team

## stigmers: the ones I follow

### /api/v1/stigmers        
          POST        -> request subscription to stigmer
              # todo: notify stigmer and wait for authorization
          GET         -> get stigmers
              # todo: subscription can be pending

## followers: the ones who follow me

### /api/v1/followers
          GET         -> get followers
### /api/v1/followers/:user_id
          PUT         -> (set authorized_at, remove forbidden_at)
          DELETE      -> remove from followers (set forbidden_at, remove authorized_at)

# todo: events

### /api/v1/events/like/:url_id
          POST        -> like url
          DELETE      -> unlink url

# todo: admin stuff
      
### /api/v1/logs ?
### /api/v1/stats ?