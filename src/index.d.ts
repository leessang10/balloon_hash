export interface BalloonHashOptions {
  /** Memory usage (number of blocks) */
  spaceCost?: number;
  /** Number of iterations */
  timeCost?: number;
  /** Degree of parallelism */
  parallelCost?: number;
}

export class BalloonHash {
  constructor(spaceCost: number, timeCost: number, parallelCost: number);
  hash(password: Uint8Array | string, salt: Uint8Array | string): Uint8Array;
}

export function balloonHash(password: Uint8Array | string, salt: Uint8Array | string, options?: BalloonHashOptions): Uint8Array;
