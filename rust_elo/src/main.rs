use std::io;

struct Player {
    elo: f64,
    rating: String,
}


fn calculate_elo(p1: f64, p2: f64, result: String) {
    let k = 32.0;
    let e1 = 1.0 / (1.0 + 10.0f64.powf((p2 - p1) / 400.0));
    let e2 = 1.0 / (1.0 + 10.0f64.powf((p1 - p2) / 400.0));
    let s1 = match result.as_str() {
    "Win" => 1.0,
    "Lose" => 0.0,
    "Draw" => 0.5,
    _ => panic!("오류"),
    };
    let s2 = match result.as_str() {
    "Win" => 0.0,
    "Lose" => 1.0,
    "Draw" => 0.5,
    _ => panic!("오류"),
    };
    let new_p1 = p1 + k * (s1 - e1);
    let new_p2 = p2 + k * (s2 - e2);
    println!("플레이어 1의 새 ELO: {}", new_p1);
    println!("플레이어 2의 새 ELO: {}", new_p2);
    }


fn main () {
	let mut p1 = Player {elo:0.0,rating:String::new()};
	let mut p2 = Player {elo:0.0,rating:String::new()};
	let mut result = String::new();

	println!("P1의 ELO를 입력하세요. :");
	io::stdin().read_line(&mut p1.rating).unwrap();
	println!("P2의 ELO를 입력하세요. :");
	io::stdin().read_line(&mut p2.rating).unwrap();
	println!("승패를 입력하세요. Win/Lose/Draw :");
	io::stdin().read_line(&mut result).unwrap();

    p1.elo = p1.rating.trim().parse::<f64>().unwrap();
    p2.elo = p2.rating.trim().parse::<f64>().unwrap();
//    구조체 X일 경우
    let result = result.trim();
    
    calculate_elo(p1.elo,p2.elo,result.to_string());
}
