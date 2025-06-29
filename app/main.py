import subprocess

from fastapi import FastAPI, Request, Form
from fastapi.responses import HTMLResponse
from fastapi.templating import Jinja2Templates
from fastapi.staticfiles import StaticFiles
import markdown
import uvicorn

app = FastAPI(title="WOD Renderer")

app.mount("/static", StaticFiles(directory="static"), name="static")
templates = Jinja2Templates(directory="templates")


@app.get("/", response_class=HTMLResponse)
async def index(request: Request):
    """Main page with empty form"""
    return templates.TemplateResponse(
        "index.html", {"request": request, "markdown_text": "", "rendered_html": ""}
    )


@app.post("/render", response_class=HTMLResponse)
async def render_markdown(request: Request, markdown_text: str = Form(...)):
    """Render WOD and return the page with results"""
    result = subprocess.run(["wod", "check", markdown_text], capture_output=True, check=False)
    if ok := result.stdout.decode("utf-8"):
        to_render = ok
    else:
        to_render = result.stderr.decode("utf-8")

    rendered_html = markdown.markdown(
        to_render, extensions=["fenced_code", "codehilite"]
    )
    return templates.TemplateResponse(
        "index.html",
        {
            "request": request,
            "markdown_text": markdown_text,
            "rendered_html": rendered_html,
        },
    )


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
