from pydantic import BaseModel, Field
from datetime import datetime
from typing import Optional

class ValidationSchema(BaseModel):
    id: int = Field(..., description="Unique identifier")
    name: str = Field(..., max_length=100, description="Name of the user")
    message: Optional[str] = Field(None, max_length=500, description="Optional message")
    greetDate: datetime = Field(..., description="Date and time of the greeting")