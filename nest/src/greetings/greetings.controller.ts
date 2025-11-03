import { Body, Controller, Get, Post } from '@nestjs/common';
import { GreetingsService } from './greetings.service';
import { GreetingsResponseDto } from './dtos/res/greetings-response.dto';
import { CreateGreetingDto } from './dtos/req/create-greeting.dto';
import { CreatedGreetingResponseDto } from './dtos/res/greeting-created-response.dto';

@Controller('greetings')
export class GreetingsController {
  constructor(private readonly greetingsService: GreetingsService) {}
  @Get()
  getGreeting(): GreetingsResponseDto {
    return this.greetingsService.getGreeting();
  }
  @Post()
  createGreeting(@Body() body: CreateGreetingDto): CreatedGreetingResponseDto {
    return this.greetingsService.createGreeting(body);
  }
}
