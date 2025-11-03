from rest_framework import serializers

class ValidationSerializer(serializers.Serializer):
    id = serializers.IntegerField()
    name = serializers.CharField(max_length=100)
    message=serializers.CharField(max_length=500,required=False)
    greetDate = serializers.DateTimeField()