import { z } from 'zod';

const TeamNameSchema = z.string().min(1).max(50);

export interface PokemonForm {
  formId: number;
  terastalType: string;
}

export class TeamError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'TeamError';
  }
}

export class Team {
  private readonly teamId: string;
  private readonly ownerId: string;
  private teamName: string;
  private pokemon: (PokemonForm | null)[];

  constructor(teamId: string, ownerId: string, teamName: string, pokemon: (PokemonForm | null)[]) {
    this.teamId = teamId;
    this.ownerId = ownerId;
    this.teamName = TeamNameSchema.parse(teamName);
    this.pokemon = pokemon.length === 6 ? pokemon : new Array(6).fill(null);
  }

  static create(teamId: string, ownerId: string, teamName: string): Team {
    return new Team(teamId, ownerId, teamName, new Array(6).fill(null));
  }

  static fromRepository(
    teamId: string,
    ownerId: string,
    teamName: string,
    pokemon: (PokemonForm | null)[]
  ): Team {
    return new Team(teamId, ownerId, teamName, pokemon);
  }

  updateName(newName: string): void {
    this.teamName = TeamNameSchema.parse(newName);
  }

  setPokemon(index: number, pokemon: PokemonForm | null): void {
    if (index < 0 || index >= 6) {
      throw new TeamError('Invalid index: must be between 0 and 5');
    }
    this.pokemon[index] = pokemon;
  }

  removePokemon(index: number): void {
    if (index < 0 || index >= 6) {
      throw new TeamError('Invalid index: must be between 0 and 5');
    }
    this.pokemon[index] = null;
  }

  addPokemon(pokemon: PokemonForm): void {
    const emptyIndex = this.pokemon.findIndex((p) => p === null);
    if (emptyIndex === -1) {
      throw new TeamError('Team is full');
    }
    this.pokemon[emptyIndex] = pokemon;
  }

  updatePokemon(newPokemon: PokemonForm[]): void {
    if (newPokemon.length > 6) {
      throw new TeamError('Too many pokemon: maximum is 6');
    }
    this.pokemon = new Array(6).fill(null);
    newPokemon.forEach((p, i) => {
      this.pokemon[i] = p;
    });
  }

  getPokemonList(): PokemonForm[] {
    return this.pokemon.filter((p): p is PokemonForm => p !== null);
  }

  getTeamId(): string {
    return this.teamId;
  }

  getOwnerId(): string {
    return this.ownerId;
  }

  getTeamName(): string {
    return this.teamName;
  }

  getPokemon(): (PokemonForm | null)[] {
    return [...this.pokemon];
  }

  getPokemonCount(): number {
    return this.pokemon.filter((p) => p !== null).length;
  }

  isFull(): boolean {
    return this.getPokemonCount() === 6;
  }
}
