import { z } from 'zod';

// Common schemas
export const UuidSchema = z.string().uuid();

// Auth DTOs
export const RegisterRequestSchema = z.object({
  username: z.string().min(1).max(20),
  email: z.string().email().max(254),
  password: z.string().min(8).max(72),
});

export const LoginRequestSchema = z.object({
  email: z.string().email(),
  password: z.string().min(1),
});

// Pokemon DTOs
export const CreatePokemonRequestSchema = z.object({
  pokemon_name: z.string(),
  pokemon_name_jp: z.string(),
  nickname: z.string().optional(),
  terastal_type: z.string(),
  ev_hp: z.number().int().min(0).max(252),
  ev_attack: z.number().int().min(0).max(252),
  ev_defense: z.number().int().min(0).max(252),
  ev_special_attack: z.number().int().min(0).max(252),
  ev_special_defense: z.number().int().min(0).max(252),
  ev_speed: z.number().int().min(0).max(252),
  iv_hp: z.number().int().min(0).max(31).default(31),
  iv_attack: z.number().int().min(0).max(31).default(31),
  iv_defense: z.number().int().min(0).max(31).default(31),
  iv_special_attack: z.number().int().min(0).max(31).default(31),
  iv_special_defense: z.number().int().min(0).max(31).default(31),
  iv_speed: z.number().int().min(0).max(31).default(31),
  nature: z.string(),
  ability: z.string(),
  held_item: z.string().optional(),
  moves: z.array(z.string()).min(1).max(4),
});

export type RegisterRequestDto = z.infer<typeof RegisterRequestSchema>;
export type LoginRequestDto = z.infer<typeof LoginRequestSchema>;
export type CreatePokemonRequestDto = z.infer<typeof CreatePokemonRequestSchema>;
