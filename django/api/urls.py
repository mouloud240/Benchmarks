from django.urls import path
from .views import sample_view

urlpatterns = [
    path('v1/greetings/', sample_view, name='sample'),
]