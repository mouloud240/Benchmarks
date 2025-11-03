import { Module } from '@nestjs/common';
import { GreetingsService } from './greetings.service';
import { GreetingsController } from './greetings.controller';

@Module({
  controllers: [GreetingsController],
  providers: [GreetingsService],
})
export class GreetingsModule {}
