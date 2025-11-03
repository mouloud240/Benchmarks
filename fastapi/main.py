from fastapi import FastAPI
from api.v1.api import router as greetings_router
app = FastAPI()
app.include_router(greetings_router, prefix="/api", tags=["greetings"])