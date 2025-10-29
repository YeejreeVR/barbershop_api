mod barbershopapi;
#[tokio::main]
async fn main() {
    add_all_to_db().await;
}
async fn print_avarage_scores_from_years() {
    let bt = barbershopapi::international_quartet_songs().await;
    for i in 1995..2026 {
        let mut all_scores_from_year = Vec::new();
        for j in bt.to_owned() {
            if j[6].parse::<i32>().unwrap() == i {
                all_scores_from_year.push(j[4].parse::<f32>().unwrap());
            }
        }
        println!("{:?}", all_scores_from_year);
        println!("{}:  {:?}",i,all_scores_from_year.iter().sum::<f32>() / all_scores_from_year.len() as f32);
    }
}
async fn add_all_to_db() {
    let wow = barbershopapi::international_chorus_in_score_order().await;
    for i in wow {
        println!("{:?}",i);
        add_song_to_db_chorus(i)
    }
    let wow = barbershopapi::international_quartet_in_score_order().await;
    for i in wow {
        println!("{:?}",i);
        add_song_to_db_quartet(i)
    }
}
fn add_song_to_db_quartet(song:Vec<String>) {
    let conn = rusqlite::Connection::open("barbershop.db").unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS quartet (song_name TEXT,quartet TEXT,year INT,music REAL,presentation REAL,singing REAL,score REAL)",[]).unwrap();
    let query = format!("INSERT INTO quartet VALUES ('{}','{}',{},{},{},{},{})",song[0],song[5],song[6].parse::<i32>().unwrap(),song[1].parse::<f32>().unwrap(),song[2].parse::<f32>().unwrap(),song[3].parse::<f32>().unwrap(),song[4].parse::<f32>().unwrap());
    println!("{}",query);
    conn.execute(&query,[]).unwrap();
    conn.close().unwrap();
}
fn add_song_to_db_chorus(song:Vec<String>) {
    let conn = rusqlite::Connection::open("barbershop.db").unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS chorus (song_name TEXT,chorus TEXT,year INT,music REAL,presentation REAL,singing REAL,score REAL)",[]).unwrap();
    let query = format!("INSERT INTO chorus VALUES ('{}','{}',{},{},{},{},{})",song[0],song[5].replace("'",""),song[6].parse::<i32>().unwrap(),song[1].parse::<f32>().unwrap(),song[2].parse::<f32>().unwrap(),song[3].parse::<f32>().unwrap(),song[4].parse::<f32>().unwrap());
    conn.execute(query.as_str(),[]).unwrap();
    conn.close().unwrap();
}