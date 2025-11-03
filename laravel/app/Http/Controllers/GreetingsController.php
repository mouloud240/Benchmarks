<?php

namespace App\Http\Controllers;

use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class GreetingsController extends Controller
{
    public function index(): JsonResponse
    {
        return response()->json(['hello word v1' => 'Hello, World!']);
    }

    public function store(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'id' => ['required', 'integer'],
            'name' => ['required', 'string'],
            'message' => ['nullable', 'string'],
            'greetDate' => ['required', 'date'],
        ]);

        return response()->json([
            'success' => true,
            'data' => $validated,
        ]);
    }
}
