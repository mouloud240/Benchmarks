package dtos

import (
	dtos "benchmark/internal/routes/dtos/req"
)

type CreatedGreetingDTO struct {
	Success bool      `json:"success"`
	Data  dtos.CreateGreetingDTO `json:"data"`

}
