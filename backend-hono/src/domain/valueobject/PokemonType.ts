import { z } from 'zod';

export const PokemonTypeSchema = z.enum([
  'Normal',
  'Fire',
  'Water',
  'Electric',
  'Grass',
  'Ice',
  'Fighting',
  'Poison',
  'Ground',
  'Flying',
  'Psychic',
  'Bug',
  'Rock',
  'Ghost',
  'Dragon',
  'Dark',
  'Steel',
  'Fairy',
]);

export type PokemonType = z.infer<typeof PokemonTypeSchema>;

export class TypeSet {
  constructor(
    public readonly type1: PokemonType,
    public readonly type2?: PokemonType
  ) {
    if (type2 && type1 === type2) {
      throw new Error('type1 and type2 cannot be the same');
    }
  }

  hasType(type: PokemonType): boolean {
    return this.type1 === type || this.type2 === type;
  }

  isSingleType(): boolean {
    return this.type2 === undefined;
  }
}
