<?php

use Illuminate\Support\Facades\Route;
use App\Http\Controllers\GreetingsController;

Route::prefix('v1')->group(function () {
    Route::get('greetings', [GreetingsController::class, 'index']);
    Route::post('greetings', [GreetingsController::class, 'store']);
});
