import { z } from 'zod';

export const StatsSchema = z.object({
  hp: z.number().int().min(0).max(252),
  attack: z.number().int().min(0).max(252),
  defense: z.number().int().min(0).max(252),
  specialAttack: z.number().int().min(0).max(252),
  specialDefense: z.number().int().min(0).max(252),
  speed: z.number().int().min(0).max(252),
});

export const IVStatsSchema = z.object({
  hp: z.number().int().min(0).max(31),
  attack: z.number().int().min(0).max(31),
  defense: z.number().int().min(0).max(31),
  specialAttack: z.number().int().min(0).max(31),
  specialDefense: z.number().int().min(0).max(31),
  speed: z.number().int().min(0).max(31),
});

export type Stats = z.infer<typeof StatsSchema>;
export type IVStats = z.infer<typeof IVStatsSchema>;

export class EVStats {
  constructor(
    public readonly hp: number,
    public readonly attack: number,
    public readonly defense: number,
    public readonly specialAttack: number,
    public readonly specialDefense: number,
    public readonly speed: number
  ) {
    const total = hp + attack + defense + specialAttack + specialDefense + speed;
    if (total > 510) {
      throw new Error('Total EVs cannot exceed 510');
    }
    StatsSchema.parse({
      hp,
      attack,
      defense,
      specialAttack,
      specialDefense,
      speed,
    });
  }

  getTotal(): number {
    return (
      this.hp + this.attack + this.defense + this.specialAttack + this.specialDefense + this.speed
    );
  }
}
