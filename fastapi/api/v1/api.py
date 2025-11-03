from fastapi import APIRouter
from .endpoint.greetings import router as greetings_router
router = APIRouter()

router.include_router(greetings_router, prefix="/v1", tags=["v1"])