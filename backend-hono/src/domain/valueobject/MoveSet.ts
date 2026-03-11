export class MoveSet {
  private readonly moves: [string, string?, string?, string?];

  constructor(move1: string, move2?: string, move3?: string, move4?: string) {
    if (!move1) {
      throw new Error('At least one move is required');
    }
    this.moves = [move1, move2, move3, move4];
  }

  getMove(index: number): string | undefined {
    if (index < 0 || index > 3) {
      throw new Error('Move index must be between 0 and 3');
    }
    return this.moves[index];
  }

  getAllMoves(): string[] {
    return this.moves.filter((move): move is string => move !== undefined);
  }

  getMoveCount(): number {
    return this.getAllMoves().length;
  }

  hasMove(moveName: string): boolean {
    return this.getAllMoves().some((move) => move === moveName);
  }
}
