import { Injectable } from '@nestjs/common';
import { GreetingsResponseDto } from './dtos/res/greetings-response.dto';

@Injectable()
export class GreetingsService {
  getGreeting(): GreetingsResponseDto {
    return { 'hello word v1': 'Hello, World!' };
  }
}
