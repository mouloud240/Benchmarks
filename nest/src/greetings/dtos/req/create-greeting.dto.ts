import { Transform } from 'class-transformer';
import {
  IsDate,
  IsNotEmpty,
  IsNumber,
  IsOptional,
  IsString,
  Min,
} from 'class-validator';

export class CreateGreetingDto {
  @IsNumber()
  @Min(1)
  readonly id: number;
  @IsString()
  @IsNotEmpty()
  readonly name: string;
  @IsOptional()
  @IsString()
  @IsNotEmpty()
  readonly message?: string;
  @Transform(({ value }) => new Date(value))
  @IsDate()
  readonly greetDate: Date;
}
