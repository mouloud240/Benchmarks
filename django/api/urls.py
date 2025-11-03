from django.urls import path
from .views import LoadTest

urlpatterns = [
    path('v1/greetings/', LoadTest.as_view(), name='loadtest'),
]