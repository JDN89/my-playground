from fastapi import FastAPI, HTTPException
from starlette.status import HTTP_404_NOT_FOUND

# entry  point
app = FastAPI()


text_posts = {"1": {"title": "New Post", "content": "cooltestpost"}}


@app.get("/posts")
def get_all_posts():
    return text_posts


@app.get("/posts/{id}")
def get_post(id: int):
    if id not in text_posts:
        return HTTPException(HTTP_404_NOT_FOUND, "Post not found")
    return text_posts.get(str(id))
