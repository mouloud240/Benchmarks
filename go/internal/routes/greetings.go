package routes

import "github.com/gin-gonic/gin"
func BindGreetingsRoutes(r *gin.Engine) {
	greetingsV1 := r.Group("api/v1/greetings")
	greetingsV1.GET("", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"Hello word v1": "Hello, World!",
		})
	})
}
