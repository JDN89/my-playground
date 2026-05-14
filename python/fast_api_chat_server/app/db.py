from typing import Annotated

from fastapi import Depends
from sqlmodel import Session, create_engine

DATABASE_URL = "postgresql+psycopg://admin:admin@localhost:5432/appdb"

engine = create_engine(DATABASE_URL, echo=True)


def get_session():
    with Session(engine) as session:
        yield session


SessionDep = Annotated[Session, Depends(get_session)]
