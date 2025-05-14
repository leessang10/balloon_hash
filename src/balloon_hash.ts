import { BalloonHash as WasmBalloonHash } from '../pkg/balloon_hash';

export class BalloonHash {
  private instance: WasmBalloonHash;

  constructor(spaceCost: number, timeCost: number, parallelCost: number) {
    this.instance = new WasmBalloonHash(spaceCost, timeCost, parallelCost);
  }

  /**
   * 주어진 비밀번호와 솔트를 사용하여 해시를 생성합니다.
   * @param password - 해시할 비밀번호
   * @param salt - 솔트 값
   * @returns 해시된 결과 (Uint8Array)
   */
  hash(password: string | Uint8Array, salt: string | Uint8Array): Uint8Array {
    const passwordBuffer = this.toUint8Array(password);
    const saltBuffer = this.toUint8Array(salt);
    return this.instance.hash(passwordBuffer, saltBuffer);
  }

  /**
   * 문자열 또는 Uint8Array를 Uint8Array로 변환합니다.
   */
  private toUint8Array(input: string | Uint8Array): Uint8Array {
    if (input instanceof Uint8Array) {
      return input;
    }
    return new TextEncoder().encode(input);
  }
}
