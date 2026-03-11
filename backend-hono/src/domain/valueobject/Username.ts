import { z } from 'zod';

export const UsernameSchema = z
  .string()
  .min(1, 'Username cannot be empty')
  .max(20, 'Username must be 20 characters or less');

export class Username {
  private readonly value: string;

  private constructor(value: string) {
    this.value = value;
  }

  static create(value: string): Username {
    const validated = UsernameSchema.parse(value);
    return new Username(validated);
  }

  getValue(): string {
    return this.value;
  }

  equals(other: Username): boolean {
    return this.value === other.value;
  }
}
