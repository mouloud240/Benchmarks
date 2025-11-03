from fastapi import APIRouter

router = APIRouter()

@router.get("/greetings")
async def read_root():
    return {"hello word v1 ": "Hello, World!"}