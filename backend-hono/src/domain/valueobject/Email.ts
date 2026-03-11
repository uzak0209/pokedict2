import { z } from 'zod';

export const EmailSchema = z.string().email('Invalid email format').max(254);

export class Email {
  private readonly value: string;

  private constructor(value: string) {
    this.value = value.toLowerCase();
  }

  static create(value: string): Email {
    const validated = EmailSchema.parse(value);
    return new Email(validated);
  }

  getValue(): string {
    return this.value;
  }

  equals(other: Email): boolean {
    return this.value === other.value;
  }
}
