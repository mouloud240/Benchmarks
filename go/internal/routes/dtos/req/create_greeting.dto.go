package dtos

import (
)
type CreateGreetingDTO struct {

	Id int `json:"id" binding:"required,omitempty,min=0"`
	Name string `json:"name" binding:"required"`
	Message *string `json:"message"`
	GreetDate string `json:"greetDate" binding:"required,datetime=2006-01-02T15:04:05Z07:00"`}
