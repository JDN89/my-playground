# ### USERS
# id
# username
# password_hash
# created_at

# ### ROOMS
# id
# name
# created_at

# ### MESSAGES
# id
# room_id
# user_id
# content
# created_at

import datetime

from sqlmodel import Field, SQLModel


class User(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    usename: str =
    password_hash: str
    created_at: datetime.datetime
