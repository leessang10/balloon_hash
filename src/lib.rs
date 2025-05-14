use wasm_bindgen::prelude::*;
use sha2::{Sha256, Digest};
use rayon::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen]
pub struct BalloonHash {
    space_cost: u32,      // 메모리 사용량 (버퍼 크기)
    time_cost: u32,       // 반복 횟수
    parallel_cost: u32,   // 병렬 처리 수준
}

#[wasm_bindgen]
impl BalloonHash {
    #[wasm_bindgen(constructor)]
    pub fn new(space_cost: u32, time_cost: u32, parallel_cost: u32) -> Result<BalloonHash, JsValue> {
        // 매개변수 검증
        if space_cost < 1 {
            return Err(JsValue::from_str("space_cost must be at least 1"));
        }
        if time_cost < 1 {
            return Err(JsValue::from_str("time_cost must be at least 1"));
        }
        if parallel_cost < 1 {
            return Err(JsValue::from_str("parallel_cost must be at least 1"));
        }

        Ok(BalloonHash {
            space_cost,
            time_cost,
            parallel_cost,
        })
    }

    pub fn hash(&self, password: &[u8], salt: &[u8]) -> Vec<u8> {
        let mut buffer = self.expand(password, salt);
        
        // 메모리 하드 믹싱 단계
        for step in 0..self.time_cost {
            self.mix(&mut buffer, step);
        }

        // 최종 해시 추출
        let mut hasher = Sha256::new();
        hasher.update(&buffer[buffer.len() - 1]);
        hasher.finalize().to_vec()
    }

    // 초기 버퍼 확장
    fn expand(&self, password: &[u8], salt: &[u8]) -> Vec<Vec<u8>> {
        let mut buffer = Vec::with_capacity(self.space_cost as usize);
        let mut hasher = Sha256::new();

        // 첫 번째 블록 생성
        hasher.update(password);
        hasher.update(salt);
        buffer.push(hasher.finalize().to_vec());

        // 나머지 블록 생성
        for i in 1..self.space_cost {
            let mut hasher = Sha256::new();
            hasher.update(&buffer[i as usize - 1]);
            buffer.push(hasher.finalize().to_vec());
        }

        buffer
    }

    // 메모리 하드 믹싱 (병렬 처리 구현)
    fn mix(&self, buffer: &mut Vec<Vec<u8>>, step: u32) {
        let chunk_size = (self.space_cost as f32 / self.parallel_cost as f32).ceil() as usize;
        
        // 버퍼를 복사하여 참조용으로 사용
        let buffer_ref = buffer.clone();
        
        // 병렬 처리를 위해 버퍼를 청크로 나누어 처리
        buffer.par_chunks_mut(chunk_size)
            .enumerate()
            .for_each(|(p, chunk)| {
                for (i, block) in chunk.iter_mut().enumerate() {
                    let absolute_index = p * chunk_size + i;
                    let mut hasher = Sha256::new();
                    
                    // 현재 블록 해시
                    hasher.update(&*block);
                    
                    // 랜덤하게 선택된 블록들과 믹스
                    for _ in 0..3 {
                        let idx = self.get_block_index(absolute_index as u32, step, buffer_ref.len() as u32);
                        hasher.update(&buffer_ref[idx as usize]);
                    }
                    
                    *block = hasher.finalize().to_vec();
                }
            });
    }

    // 블록 인덱스 계산 (RFC 9197의 수식에 따름)
    fn get_block_index(&self, current: u32, step: u32, total_blocks: u32) -> u32 {
        let mut hasher = Sha256::new();
        hasher.update(current.to_le_bytes());
        hasher.update(step.to_le_bytes());
        
        let result = hasher.finalize();
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&result[0..4]);
        
        u32::from_le_bytes(bytes) % total_blocks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_basic_hash() {
        let balloon = BalloonHash::new(16, 20, 4).unwrap();  // parallel_cost를 4로 설정
        let password = b"password123";
        let salt = b"salt123";
        
        let hash = balloon.hash(password, salt);
        assert_eq!(hash.len(), 32); // SHA-256 출력 크기
    }

    #[wasm_bindgen_test]
    fn test_parameter_validation() {
        assert!(BalloonHash::new(0, 20, 4).is_err());  // invalid space_cost
        assert!(BalloonHash::new(16, 0, 4).is_err());  // invalid time_cost
        assert!(BalloonHash::new(16, 20, 0).is_err()); // invalid parallel_cost
    }

    #[wasm_bindgen_test]
    fn test_different_parameters() {
        let balloon1 = BalloonHash::new(16, 20, 4).unwrap();
        let balloon2 = BalloonHash::new(32, 20, 4).unwrap();
        
        let password = b"password123";
        let salt = b"salt123";
        
        let hash1 = balloon1.hash(password, salt);
        let hash2 = balloon2.hash(password, salt);
        
        // 다른 매개변수로 생성된 해시는 달라야 함
        assert_ne!(hash1, hash2);
    }
}
