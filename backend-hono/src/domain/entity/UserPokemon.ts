import { TypeSet, PokemonType } from '../valueobject/PokemonType';
import { EVStats, IVStats } from '../valueobject/Stats';
import { Nature } from '../valueobject/Nature';
import { MoveSet } from '../valueobject/MoveSet';

export interface UserPokemonProps {
  pokemonId: string;
  userId: string;
  nickname?: string;
  formId: number;
  speciesId: number;
  fullname: string;
  fullnameJp: string;
  typeset: TypeSet;
  terastalType: PokemonType;
  ev: EVStats;
  iv: IVStats;
  nature: Nature;
  ability: string;
  heldItem?: string;
  moves: MoveSet;
  createdAt: Date;

  // Japanese translations (joined data)
  natureJp?: string;
  abilityJp?: string;
  heldItemJp?: string;
  movesJp: (string | undefined)[];
  movesTypes: (string | undefined)[];
}

export class UserPokemon {
  constructor(private readonly props: UserPokemonProps) {}

  static create(
    pokemonId: string,
    userId: string,
    formId: number,
    speciesId: number,
    fullname: string,
    fullnameJp: string,
    typeset: TypeSet,
    terastalType: PokemonType,
    ev: EVStats,
    iv: IVStats,
    nature: Nature,
    ability: string,
    moves: MoveSet,
    nickname?: string,
    heldItem?: string
  ): UserPokemon {
    return new UserPokemon({
      pokemonId,
      userId,
      nickname,
      formId,
      speciesId,
      fullname,
      fullnameJp,
      typeset,
      terastalType,
      ev,
      iv,
      nature,
      ability,
      heldItem,
      moves,
      createdAt: new Date(),
      movesJp: [],
      movesTypes: [],
    });
  }

  static fromRepository(props: UserPokemonProps): UserPokemon {
    return new UserPokemon(props);
  }

  updateNickname(nickname?: string): UserPokemon {
    return new UserPokemon({
      ...this.props,
      nickname,
    });
  }

  updateBattleInfo(
    terastalType: PokemonType,
    ev: EVStats,
    iv: IVStats,
    nature: Nature,
    ability: string,
    moves: MoveSet,
    heldItem?: string
  ): UserPokemon {
    return new UserPokemon({
      ...this.props,
      terastalType,
      ev,
      iv,
      nature,
      ability,
      heldItem,
      moves,
    });
  }

  getPokemonId(): string {
    return this.props.pokemonId;
  }

  getUserId(): string {
    return this.props.userId;
  }

  getNickname(): string | undefined {
    return this.props.nickname;
  }

  getFormId(): number {
    return this.props.formId;
  }

  getSpeciesId(): number {
    return this.props.speciesId;
  }

  getFullname(): string {
    return this.props.fullname;
  }

  getFullnameJp(): string {
    return this.props.fullnameJp;
  }

  getTypeset(): TypeSet {
    return this.props.typeset;
  }

  getTerastalType(): PokemonType {
    return this.props.terastalType;
  }

  getEV(): EVStats {
    return this.props.ev;
  }

  getIV(): IVStats {
    return this.props.iv;
  }

  getNature(): Nature {
    return this.props.nature;
  }

  getAbility(): string {
    return this.props.ability;
  }

  getHeldItem(): string | undefined {
    return this.props.heldItem;
  }

  getMoves(): MoveSet {
    return this.props.moves;
  }

  getCreatedAt(): Date {
    return this.props.createdAt;
  }

  // Japanese translations
  getNatureJp(): string | undefined {
    return this.props.natureJp;
  }

  getAbilityJp(): string | undefined {
    return this.props.abilityJp;
  }

  getHeldItemJp(): string | undefined {
    return this.props.heldItemJp;
  }

  getMovesJp(): (string | undefined)[] {
    return this.props.movesJp;
  }

  getMovesTypes(): (string | undefined)[] {
    return this.props.movesTypes;
  }
}
