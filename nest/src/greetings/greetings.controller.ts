import { Controller, Get } from '@nestjs/common';
import { GreetingsService } from './greetings.service';
import { GreetingsResponseDto } from './dtos/res/greetings-response.dto';

@Controller('greetings')
export class GreetingsController {
  constructor(private readonly greetingsService: GreetingsService) {}
  @Get()
  getGreeting(): GreetingsResponseDto {
    return this.greetingsService.getGreeting();
  }
}
