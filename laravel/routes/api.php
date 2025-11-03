<?php

use Illuminate\Support\Facades\Route;
use App\Http\Controllers\GreetingsController;

// Note: routes in this file are auto-prefixed with '/api'
Route::prefix('v1')->group(function () {
    Route::get('greetings', [GreetingsController::class, 'index']);
    Route::post('greetings', [GreetingsController::class, 'store']);
});
