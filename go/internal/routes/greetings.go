package routes

import (
	dtos "benchmark/internal/routes/dtos/req"
	dtos_res "benchmark/internal/routes/dtos/res"

	"github.com/gin-gonic/gin"
)
func BindGreetingsRoutes(r *gin.Engine) {
	greetingsV1 := r.Group("api/v1/greetings")
	greetingsV1.GET("", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"Hello word v1": "Hello, World!",
		})
	})
	greetingsV1.POST("", func(c *gin.Context) {

		var body  dtos.CreateGreetingDTO
		if err := c.ShouldBindJSON(&body); err != nil {
			c.JSON(400, gin.H{
				"error": err.Error(),
			})
			return
		}
		response:= dtos_res.CreatedGreetingDTO{
			Success: true,
			Data: body,
		}
		c.JSON(201, response)
		})
}
