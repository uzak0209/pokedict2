import * as v from 'valibot';
import { get } from './client';
import type { PokemonMasterDto, PokemonMasterResponseDto } from '../types/api';

// Valibot schema for Pokemon Master Data
const PokemonMasterSchema = v.object({
    form_id: v.number(),
    species_id: v.number(),
    fullname: v.string(),
    fullname_ja: v.nullable(v.string()),
    type1: v.string(),
    type2: v.nullable(v.string()),
    hp: v.number(),
    attack: v.number(),
    defense: v.number(),
    sp_attack: v.number(),
    sp_defense: v.number(),
    speed: v.number(),
    usage: v.nullable(v.number()),
    raw_count: v.nullable(v.number()),
});

const PokemonMasterResponseSchema = v.object({
    pokemon: v.array(PokemonMasterSchema),
    total: v.number(),
});

// Usage Stats Schemas
const UsageDetailSchema = v.object({
    name: v.string(),
    name_ja: v.nullable(v.string()),
    type: v.optional(v.nullable(v.string())),
    percentage: v.number(),
});

const PokemonUsageStatsSchema = v.object({
    form_id: v.number(),
    moves: v.array(UsageDetailSchema),
    items: v.array(UsageDetailSchema),
    abilities: v.array(UsageDetailSchema),
    tera_types: v.array(UsageDetailSchema),
    natures: v.array(UsageDetailSchema),
});

export type UsageDetailDto = v.InferOutput<typeof UsageDetailSchema>;
export type PokemonUsageStatsDto = v.InferOutput<typeof PokemonUsageStatsSchema>;

/**
 * 全ポケモンマスタデータを取得（使用率順）
 */
export async function getAllPokemonMaster(): Promise<PokemonMasterResponseDto> {
    return get('/pokemon/master', PokemonMasterResponseSchema) as unknown as Promise<PokemonMasterResponseDto>;
}

/**
 * 使用率トップポケモンを取得
 */
export async function getTopPokemonMaster(limit: number = 50): Promise<PokemonMasterResponseDto> {
    return get(`/pokemon/master/top?limit=${limit}`, PokemonMasterResponseSchema) as unknown as Promise<PokemonMasterResponseDto>;
}

/**
 * ポケモンの詳細使用率統計を取得
 */
export async function getPokemonUsage(formId: number): Promise<PokemonUsageStatsDto> {
    return get(`/pokemon/master/${formId}/usage`, PokemonUsageStatsSchema);
}
