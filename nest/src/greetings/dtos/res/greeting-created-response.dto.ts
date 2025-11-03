import { CreateGreetingDto } from '../req/create-greeting.dto';

export class CreatedGreetingResponseDto {
  //NOTE : this should be done on an interceptor level not on pipe but for simplicity we are doing it here
  readonly data: CreateGreetingDto;
  readonly success: boolean;
}
