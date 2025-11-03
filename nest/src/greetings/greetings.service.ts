import { Injectable } from '@nestjs/common';
import { GreetingsResponseDto } from './dtos/res/greetings-response.dto';
import { CreatedGreetingResponseDto } from './dtos/res/greeting-created-response.dto';
import { CreateGreetingDto } from './dtos/req/create-greeting.dto';

@Injectable()
export class GreetingsService {
  getGreeting(): GreetingsResponseDto {
    return { 'hello word v1': 'Hello, World!' };
  }
  createGreeting(greeting: CreateGreetingDto): CreatedGreetingResponseDto {
    return {
      data: greeting,
      success: true,
    };
  }
}
