use cw_storage_plus::Item;

pub const COUNTER: Item<u64> = Item::new("counter");

// lib.rs -> client 가 호출하는 파트

// lib.rs -> contract.rs 내부 호출파트
// contract.rs -> 내부 비즈니스 로직

// state.rs -> 컨트랙트에 저장할 데이터 객체를 정의하게 됨.
// msg.rs ->

// /post -> UpdateCounterState()
// /get

// /delete
