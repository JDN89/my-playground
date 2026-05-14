# Practice app - chat server

Het is de eerste keer dat ik met FastsApi werk. Je kan al veel met AI doen en uitvoeren, maar voor mij is de beste manier om iets (een framework) er iets mee te maken zonder hulp van AI.
Ik ben wel van plan, eens ik deze kleine app geschreven heb, om mijn werk te laten reviewn door AI en zo te zien waar ik nog kon verbeteren.

## Chat app


## DB model

### USERS
id
username
password_hash
created_at

### ROOMS
id
name
created_at

### MESSAGES
id
room_id
user_id
content
created_at

## API

### AUTH
POST /register
POST /login

### ROOMS
GET /rooms
POST /rooms

### MESSAGES
GET /rooms/{room_id}/messages
POST /rooms/{room_id}/messages

### WEBSOCKET endpoint
look into, never implemented websockets before.

# TODO
- [ ] health endpoint
- [ ] db connection
- [ ] SqlAlchemy Models user, room, message
- [ ] migraitons (Alembic)
- [ ] CRUD api's
- [ ] Websockts
- [ ] auth
  - [ ] password hashing
  - [ ] JWT tokens
  - [ ] auth ws connections
