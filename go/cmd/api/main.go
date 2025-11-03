package main

import (
	"benchmark/internal/routes"

	"github.com/gin-gonic/gin"
)

func main() {
	r := gin.Default()
	addr:= ":8080"
  routes.SetupRoutes(r)
	r.SetTrustedProxies(nil)
	r.Run(addr);
	

	
}
