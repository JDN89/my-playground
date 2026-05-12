from queue import Full

from fastapi import FastAPI
from pydantic import BaseModel

app = FastAPI()


class FullName(BaseModel):
    last_name: str
    first_name: str
    full_name: str | None


@app.get("/")
async def root():
    return {"message: Hellow world!"}


@app.post("/name/{id}")
async def get_full_name(id: int, name: FullName, q: str | None) -> str:
    print(f"query_param : {q}")
    print(id)
    name.full_name = name.first_name + " " + name.last_name
    return name.full_name


@app.get("/names")
async def get_names(names: list[str]):
    for name in names:
        print(name.capitalize)
