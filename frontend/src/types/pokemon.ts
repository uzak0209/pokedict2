// Pokemon types
export type PokemonType =
  | 'Normal' | 'Fire' | 'Water' | 'Grass' | 'Electric' | 'Ice'
  | 'Fighting' | 'Poison' | 'Ground' | 'Flying' | 'Psychic' | 'Bug'
  | 'Rock' | 'Ghost' | 'Dragon' | 'Dark' | 'Steel' | 'Fairy';

// Team building strategy: 軸4体 + 補完2体
export type TeamRole =
  | 'core'         // 軸ポケモン（4体）
  | 'support';     // 補完ポケモン（2体）

// 有利不利の判定
export type Matchup = 'advantage' | 'neutral' | 'disadvantage' | 'unknown';

// ポケモンの基本情報
export interface Pokemon {
  id: string;
  name: string;
  nameJp: string;
  types: [PokemonType] | [PokemonType, PokemonType];
  ability?: string;
  item?: string;
  sprite?: string;
  stats?: {
    hp: number;
    attack: number;
    defense: number;
    spAttack: number;
    spDefense: number;
    speed: number;
  };
}

// チームメンバー
export interface TeamMember {
  pokemon: Pokemon;
  role: TeamRole;
  moves?: string[];
  evs?: {
    hp: number;
    attack: number;
    defense: number;
    spAttack: number;
    spDefense: number;
    speed: number;
  };
  notes?: string;
}

// チーム構成
export interface Team {
  id: string;
  name: string;
  core: TeamMember[];      // 軸4体（最大4）
  support: TeamMember[];   // 補完2体（最大2）
  createdAt: Date;
  updatedAt: Date;
}

// 対戦相手との相性情報
export interface MatchupData {
  myPokemon: string;       // 自分のポケモンID
  opponent: string;        // 相手のポケモンID
  matchup: Matchup;        // 有利/不利
  confidence: number;      // 信頼度 (0-1)
  userFeedback?: Matchup;  // ユーザーの修正
  source: 'calculated' | 'user' | 'aggregated';
}

// Top30ポケモン（使用率上位）
export interface Top30Pokemon extends Pokemon {
  rank: number;
  usageRate: number;
}
