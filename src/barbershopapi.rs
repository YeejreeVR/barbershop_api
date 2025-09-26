#[allow(dead_code)]
const INTERNATIONAL_QUARTET_CONTEST_IDS: [i32; 30] = [277,276,275,257,173,108,76,8,125,207,224,321,359,401,459,503,526,575,610,702,738,774,810,846,882,918,953,995,1013,1087];
const INTERNATIONAL_CHORUS_CONTEST_IDS: [i32; 30] = [224,223,222,221,204,187,136,119,70,69,68,1,2,3,137,154,225,243,261,279,297,322,340,375,393,411,429,451,468,488];
fn get_lowest_score(bhtcaq: Vec<Vec<String>>) -> String {
    let mut return_value:String = String::from("100.0");
    let mut lowest_score:f32 = 100.0;
    for i in bhtcaq {
        println!("{:?}",i);
        if i[4].parse::<f32>().unwrap() < lowest_score {
            return_value = i.get(0).unwrap().to_string();
            lowest_score = i[4].parse::<f32>().unwrap();
        }
    }
    return_value

}
fn songs_in_score_order(bhtcaq: std::collections::BTreeMap<i32,Vec<String>>) -> Vec<String> {
    let mut return_value:Vec<String> = Vec::new();
    for i in bhtcaq.values() {
        for j in i {
            return_value.push(j.to_string());
        }
    }
    return_value.reverse();
    return_value
}
fn get_highest_score(bhtcaq: Vec<Vec<String>>) -> String {
    let mut return_value:String = String::from("0.0");
    let mut highest_score:f32 = 0.0;
    for i in bhtcaq {
        if i[4].parse::<f32>().unwrap() > highest_score {
            return_value = i.get(0).unwrap().to_string();
            highest_score = i[4].parse::<f32>().unwrap();
        }
    }
    return_value
}
fn make_btreemap_title_score(bhtcaq: Vec<Vec<String>>) -> std::collections::BTreeMap<i32, Vec<String>>{
    let mut return_value: std::collections::BTreeMap<i32, Vec<String>> = std::collections::BTreeMap::new();
    for i in bhtcaq {
        let key = (i[4].parse::<f32>().unwrap() * 10.0) as i32;
        let title = i.get(0).unwrap().to_string();
        return_value.entry(key).or_default().push(title);
    }
    return_value
}
fn make_btreemap_title_presentation(bhtcaq: Vec<Vec<String>>) -> std::collections::BTreeMap<i32, Vec<String>>{
    let mut return_value: std::collections::BTreeMap<i32, Vec<String>> = std::collections::BTreeMap::new();
    for i in bhtcaq {
        let key = (i[2].parse::<f32>().unwrap() * 10.0) as i32;
        let title = i.get(0).unwrap().to_string();
        return_value.entry(key).or_default().push(title);
    }
    return_value
}
fn make_btreemap_title_music(bhtcaq: Vec<Vec<String>>) -> std::collections::BTreeMap<i32, Vec<String>>{
    let mut return_value: std::collections::BTreeMap<i32, Vec<String>> = std::collections::BTreeMap::new();
    for i in bhtcaq {
        let key = (i[1].parse::<f32>().unwrap() * 10.0) as i32;
        let title = i.get(0).unwrap().to_string();
        return_value.entry(key).or_default().push(title);
    }
    return_value
}
fn make_btreemap_title_singing(bhtcaq: Vec<Vec<String>>) -> std::collections::BTreeMap<i32, Vec<String>>{
    let mut return_value: std::collections::BTreeMap<i32, Vec<String>> = std::collections::BTreeMap::new();
    for i in bhtcaq {
        let key = (i[3].parse::<f32>().unwrap() * 10.0) as i32;
        let title = i.get(0).unwrap().to_string();
        return_value.entry(key).or_default().push(title);
    }
    return_value
}
fn make_btreemap_scores_score(bhtcaq: Vec<Vec<String>>) -> std::collections::BTreeMap<i32, Vec<Vec<String>>>{
    let mut return_value: std::collections::BTreeMap<i32, Vec<Vec<String>>> = std::collections::BTreeMap::new();
    for i in bhtcaq {
        let key = (i[4].parse::<f32>().unwrap() * 10.0) as i32;
        return_value.entry(key).or_default().push(i);
    }
    return_value
}
async fn get_scores_from_list(list: Vec<i32>) -> Vec<Vec<String>> {
    let mut return_value:Vec<Vec<String>> = Vec::new();
    for i in list {

        for song in get_scores_from_link(&format!("https://www.bsmdb.com/Quartet/Contest/{}",i)).await {return_value.push(song)}
    }
    return_value
}
async fn get_scores_from_list_threaded(ids: Vec<i32>) -> Vec<Vec<String>> {
    const BASE_URL: &str = "https://www.bsmdb.com/Quartet/Contest/";

    let mut handles = Vec::new();
    for small_list in split_list(ids, 10) {
        handles.push(tokio::spawn(async move {
            let mut group_results: Vec<Vec<String>> = Vec::new();
            for id in small_list {
                let url = format!("{BASE_URL}{id}");
                let mut scores = new_get_scores_from_link(&url).await;
                group_results.append(&mut scores);
            }
            group_results
        }));
    }

    let mut results: Vec<Vec<String>> = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(mut group) => results.append(&mut group),
            Err(e) => {
                eprintln!("worker task failed: {e}");
            }
        }
    }
    results
}
pub async fn new_get_scores_from_link(link:&str) -> Vec<Vec<String>>{
    let mut return_value:Vec<Vec<String>> = Vec::new();
    let request = reqwest::get(link).await.unwrap().text().await.unwrap();
    let html = scraper::Html::parse_document(&request);
    let scores_selector = scraper::Selector::parse("td").unwrap();
    let scores = html.select(&scores_selector);
    let mut ne_return_value:Vec<String> = Vec::new();
    let mut quartet_name:String = String::new();
    for i in scores {
        if i.attr("class").is_some() {
            if i.attr("class").unwrap() == "quartetLine" && i.child_elements().count() > 0 {
                quartet_name = i.child_elements().next().unwrap().inner_html();
            }
        }
        if i.attr("class").is_some() {
            if i.attr("class").unwrap() == "scoreLine" {

                if i.child_elements().count() > 0 {

                    let song_name = i.child_elements().next().unwrap().inner_html();
                    ne_return_value.push(song_name);
                    //println!("{:?}",score.child_elements().next().unwrap().inner_html());
                }
                else {
                    //println!("{:?}",score.inner_html());
                    ne_return_value.push(i.inner_html());
                }
                if ne_return_value.len() > 4 {
                    ne_return_value.push(quartet_name.clone());
                    return_value.push(ne_return_value);
                    ne_return_value = Vec::new();
                }
            }
        }


    }
    return_value
}
pub async fn get_scores_from_link(link:&str) -> Vec<Vec<String>>{
    let mut return_value:Vec<Vec<String>> = Vec::new();
    let request = reqwest::get(link).await.unwrap().text().await.unwrap();
    let html = scraper::html::Html::parse_document(&request);
    let scores_selector = scraper::Selector::parse("td.scoreLine").unwrap();
    let scores = html.select(&scores_selector);
    let mut ne_return_value:Vec<String> = Vec::new();

    for score in scores {

        if score.child_elements().count() > 0 {

            let song_name = score.child_elements().next().unwrap().inner_html();
            ne_return_value.push(song_name);
            //println!("{:?}",score.child_elements().next().unwrap().inner_html());
        }
        else {
            //println!("{:?}",score.inner_html());
            ne_return_value.push(score.inner_html());
        }
        if ne_return_value.len() > 4 {
            return_value.push(ne_return_value);
            ne_return_value = Vec::new();
        }
    }
    return_value
}
async fn babashap_haircut_that_costs_a_quauta() -> Vec<Vec<String>> {
    let mut return_value:Vec<Vec<String>> = Vec::new();
    for i in 1..1089 {
        println!("{:?}",i);
        for ve in get_scores_from_link(&format!("https://www.bsmdb.com/Quartet/Contest/{}",i)).await {return_value.push(ve); }

    }
    return_value
}
fn split_list(list: Vec<i32>, slices:i32) -> Vec<Vec<i32>> {
    let mut return_value:Vec<Vec<i32>> = Vec::new();
    let mut changing_list = list;
    let split_size = changing_list.len() / slices as usize;
    loop {
        if changing_list.len() <= split_size + 1 {
            return_value.push(changing_list.clone());
            break;
        }
        else {
            let split = changing_list.split_at(split_size);
            return_value.push(split.0.to_vec());
            changing_list = split.1.to_vec();
        }
        if changing_list.len() <= 1 {
            break;
        }
    }
    return_value
}
async fn get_scores_from_list_threaded_chorus(ids: Vec<i32>) -> Vec<Vec<String>> {
    const BASE_URL: &str = "https://www.bsmdb.com/Chorus/Contest/";

    let mut handles = Vec::new();
    for small_list in split_list(ids, 10) {
        handles.push(tokio::spawn(async move {
            let mut group_results: Vec<Vec<String>> = Vec::new();
            for id in small_list {
                let url = format!("{BASE_URL}{id}");
                let mut scores = new_get_scores_from_link(&url).await;
                group_results.append(&mut scores);
            }
            group_results
        }));
    }

    let mut results: Vec<Vec<String>> = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(mut group) => results.append(&mut group),
            Err(e) => {
                eprintln!("worker task failed: {e}");
            }
        }
    }
    results
}
fn songs_in_score_order_all(bhtcaq: std::collections::BTreeMap<i32, Vec<Vec<String>>>) -> Vec<Vec<String>> {
    let mut return_value:Vec<Vec<String>> = Vec::new();
    for i in bhtcaq.values() {
        for j in i {
            return_value.push(j.clone());
        }
    }
    return_value.reverse();
    return_value
}
pub async fn international_quartet_in_score_order() -> Vec<Vec<String>> {
    let btreemap:std::collections::BTreeMap<i32,Vec<Vec<String>>> = make_btreemap_scores_score(get_scores_from_list_threaded(INTERNATIONAL_QUARTET_CONTEST_IDS.to_vec()).await.to_vec());
    songs_in_score_order_all(btreemap)
}
pub async fn international_chorus_in_score_order() -> Vec<Vec<String>> {
    let btreemap:std::collections::BTreeMap<i32,Vec<Vec<String>>> = make_btreemap_scores_score(get_scores_from_list_threaded_chorus(INTERNATIONAL_CHORUS_CONTEST_IDS.to_vec()).await);
    songs_in_score_order_all(btreemap)
}
pub async fn international_quartet_songs() -> Vec<Vec<String>> {
    get_scores_from_list_threaded(INTERNATIONAL_QUARTET_CONTEST_IDS.to_vec()).await
}
pub async fn international_chorus_songs() -> Vec<Vec<String>> {
    get_scores_from_list_threaded_chorus(INTERNATIONAL_CHORUS_CONTEST_IDS.to_vec()).await
}
pub async fn create_btreemap_international_quartet_songs(score:&str) -> std::collections::BTreeMap<i32,Vec<String>> {
    let score = score.to_lowercase();
    if score == "total".to_string() {
        make_btreemap_title_score(get_scores_from_list_threaded(INTERNATIONAL_QUARTET_CONTEST_IDS.to_vec()).await)
    }
    else if score == "presentation".to_string() || score == "prs" {
        make_btreemap_title_presentation(get_scores_from_list_threaded(INTERNATIONAL_QUARTET_CONTEST_IDS.to_vec()).await)
    }
    else if score == "music".to_string() || score == "mus" {
        make_btreemap_title_music(get_scores_from_list_threaded(INTERNATIONAL_QUARTET_CONTEST_IDS.to_vec()).await)
    }
    else if score == "singing".to_string() || score == "sng" {
        make_btreemap_title_singing(get_scores_from_list_threaded(INTERNATIONAL_QUARTET_CONTEST_IDS.to_vec()).await)
    }
    else {
        make_btreemap_title_score(get_scores_from_list_threaded(INTERNATIONAL_QUARTET_CONTEST_IDS.to_vec()).await)
    }
}
pub async fn create_btreemap_international_chorus_songs(score:&str) -> std::collections::BTreeMap<i32,Vec<String>> {
    let score = score.to_lowercase();
    if score == "total".to_string() {
        make_btreemap_title_score(get_scores_from_list_threaded_chorus(INTERNATIONAL_CHORUS_CONTEST_IDS.to_vec()).await)
    }
    else if score == "presentation".to_string() || score == "prs" {
        make_btreemap_title_presentation(get_scores_from_list_threaded_chorus(INTERNATIONAL_CHORUS_CONTEST_IDS.to_vec()).await)
    }
    else if score == "music".to_string() || score == "mus" {
        make_btreemap_title_music(get_scores_from_list_threaded_chorus(INTERNATIONAL_CHORUS_CONTEST_IDS.to_vec()).await)
    }
    else if score == "singing".to_string() || score == "sng" {
        make_btreemap_title_singing(get_scores_from_list_threaded_chorus(INTERNATIONAL_CHORUS_CONTEST_IDS.to_vec()).await)
    }
    else {
        make_btreemap_title_score(get_scores_from_list_threaded_chorus(INTERNATIONAL_CHORUS_CONTEST_IDS.to_vec()).await)
    }
}
