struct Package {
    weight: f64,
}

impl Package {
    fn new(weight: f64) -> Self {
        Self { weight }
    }
    // 테스트용. self 구조체에서 weight를 변경하기 때문에 &mut self를 사용해야함.
    fn change_weight(&mut self) {
        self.weight = 10.0;
    }
}

// Default trait의 구현
// 패키지 생성 시 입력이 없는 경우를 대비한 것.
impl Default for Package {
    fn default() -> Self {
        Self { weight: 3.0 }
    }
}
fn main() {
    let p = Package::default();
    // 테스트용 pa 스트럭처가 변경될 것이기 때문에 mut pa를 사용.
    let mut pa = Package::new(5.0);
    // 여기서 패키지 스트럭처의 내용이 변경되기 때문에 위에 있는 pa를 mut로 해야함.
    pa.change_weight();
}