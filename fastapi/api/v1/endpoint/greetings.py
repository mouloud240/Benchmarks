from fastapi import APIRouter
from schemas.validation import ValidationSchema
router = APIRouter()

@router.get("/greetings")
async def read_root():
    return {"hello word v1 ": "Hello, World!"}

@router.post("/greetings/")
async def validate_data(data: dict):
    try:
        validated_data = ValidationSchema(**data)
        return {"status": "success", "data": validated_data.dict()}
    except Exception as e:
        return {"status": "error", "errors": e.errors()}