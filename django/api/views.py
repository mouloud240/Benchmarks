from rest_framework.response import Response
from rest_framework.decorators import  APIView

class LoadTest(APIView):
    def get(self, request):
        return Response({"hello word v1 ": "Hello, World!"})

    def post(self, request):
        from .serlaizers import ValidationSerializer
        serializer = ValidationSerializer(data=request.data)
        if serializer.is_valid():
            return Response({"status": "success", "data": serializer.validated_data})
        else:
            return Response({"status": "error", "errors": serializer.errors}, status=400)