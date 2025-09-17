pub const international_contest_ids: [i32; 30] = [277,276,275,257,173,108,76,8,125,207,224,321,359,401,459,503,526,575,610,702,738,774,810,846,882,918,953,995,1013,1087];
#[tokio::main]
async fn main() {
    //println!("{:?}", songs_in_score_order(make_btreemap_title_score(get_scores_from_list(international_contest_ids.to_vec()).await)));
    println!("{:?}",songs_in_score_order(make_btreemap_title_score(get_scores_from_link("https://www.bsmdb.com/Quartet/Contest/1087").await)));
    //println!("{:?}",songs_in_score_order(babashap_haircut_that_costs_a_quauta().await));
    //println!("{:?}",songs_in_score_order(make_btreemap_title_singing(get_scores_from_link("https://www.bsmdb.com/Quartet/Contest/995").await)));

}
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
fn songs_in_score_order(bhtcaq: std::collections::BTreeMap<i32,String>) -> Vec<String> {
    let mut return_value:Vec<String> = Vec::new();
    for i in bhtcaq.values() {
        return_value.push(i.to_string());
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
fn make_btreemap_title_score(bhtcaq: Vec<Vec<String>>) -> std::collections::BTreeMap<i32,String>{
    let mut return_value:std::collections::BTreeMap<i32,String> = std::collections::BTreeMap::new();
    for i in bhtcaq {
        println!("{:?}",i.get(0).unwrap());
        return_value.insert((i[4].parse::<f32>().unwrap()*10.0) as i32,i.get(0).unwrap().to_string());

    }
    return_value
}
fn make_btreemap_title_singing(bhtcaq: Vec<Vec<String>>) -> std::collections::BTreeMap<i32,String>{
    let mut return_value:std::collections::BTreeMap<i32,String> = std::collections::BTreeMap::new();
    for i in bhtcaq {
        println!("{:?}",i.get(0).unwrap());
        return_value.insert((i[3].parse::<f32>().unwrap()*10.0) as i32,i.get(0).unwrap().to_string());

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
async fn get_scores_from_link(link:&str) -> Vec<Vec<String>>{
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