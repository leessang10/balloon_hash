import { BalloonHash } from './balloon_hash';

async function example() {
  // BalloonHash 인스턴스 생성
  const balloonHash = new BalloonHash(16, 20, 4);

  // 문자열로 해시 생성
  const password = 'my_secure_password';
  const salt = 'random_salt_value';
  const hash1 = balloonHash.hash(password, salt);
  console.log('String input hash:', Buffer.from(hash1).toString('hex'));

  // Uint8Array로 해시 생성
  const passwordBuffer = new TextEncoder().encode(password);
  const saltBuffer = new TextEncoder().encode(salt);
  const hash2 = balloonHash.hash(passwordBuffer, saltBuffer);
  console.log('Uint8Array input hash:', Buffer.from(hash2).toString('hex'));
}

example().catch(console.error);
