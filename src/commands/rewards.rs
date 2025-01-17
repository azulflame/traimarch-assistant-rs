use std::cmp::min;
use std::str::FromStr;
use rand::{random, Rng};
use crate::common::{Context, Error};
use crate::config::{get_config_val, SecretType};

pub enum Difficulty {
    VeryDeadly,
    Deadly,
    Hard,
    Normal,
    Easy
}


impl Difficulty {
    fn from_str(s: &str) -> Difficulty {
        let mut closest = Difficulty::Easy;

        for x in vec![Difficulty::Easy, Difficulty::Normal, Difficulty::Hard, Difficulty::Deadly, Difficulty::VeryDeadly] {
            if x.get_text().to_lowercase().contains(s.to_lowercase().as_str()) || s.to_string().to_lowercase().contains(x.get_text().to_lowercase().as_str())
            {
                closest = x;
            }
        }
        closest
    }
    fn get_text(&self) -> String {
        match self {
            Difficulty::VeryDeadly => "TPK likely",
            Difficulty::Deadly => "Deadly",
            Difficulty::Hard => "Hard",
            Difficulty::Normal => "Normal",
            Difficulty::Easy => "Easy",
        }
            .to_string()
    }
    fn get_index(&self) -> usize {
        match self {
            Difficulty::VeryDeadly => 0,
            Difficulty::Deadly => 1,
            Difficulty::Hard => 2,
            Difficulty::Normal => 3,
            Difficulty::Easy => 4,
        }
    }
}

pub async fn difficulty_autocomplete(_ctx: Context<'_>, partial: &str) -> Vec<String>
{
    vec![Difficulty::Easy, Difficulty::Normal, Difficulty::Hard, Difficulty::Deadly, Difficulty::VeryDeadly]
        .iter().filter(|x| x.get_text().to_lowercase().contains(partial.to_lowercase().as_str())).map(|x| x.get_text()).collect()
}

#[poise::command(slash_command)]
pub async fn rewards(ctx: Context<'_>,

                     #[description = "The time the quest took"]
                     time: f32,
                     #[description = "The number of t1 players"]
                     #[min = 0]
                     #[max = 20]
                     t1: Option<u32>,
                     #[description = "The number of t2 players"]
                     #[min = 0]
                     #[max = 20]
                     t2: Option<u32>,
                     #[description = "The number of t3 players"]
                     #[min = 0]
                     #[max = 20]
                     t3: Option<u32>,
                     #[description = "The number of t4 players"]
                     #[min = 0]
                     #[max = 20]
                     t4: Option<u32>,
                     #[description = "The difficulty of the quest"]
                     #[autocomplete = "difficulty_autocomplete"]
                     difficulty: String,
                     #[description = "If the quest was a VC quest"]
                     vc: bool
) -> Result<(), Error> {
    let diff = Difficulty::from_str(difficulty.as_str());
    let _tier: f32 = get_overall_tier(t1.clone(), t2.clone(), t3.clone(), t4.clone());
    let vc_gold_max = [28, 25, 20, 12, 6];
    let vc_gold_min = [26, 21, 15, 12, 6];
    let pbp_gold_max = [28, 25, 15, 0, -2];
    let pbp_gold_min = [26, 20, 15, 0, -2];
    let vc_stamps = [5,4,2,1,0];
    let pbp_stamps = [4, 3, 1, 0, -1];
    let dt_mult = [2.5, 2.0, 1.5, 1.0, 0.75];

    let int_time: i32 = (time.round() + 0.1) as i32;

    let stamps = match vc {
        true => vc_stamps,
        false => pbp_stamps
    }[diff.get_index()] + int_time;
    let gold_min = match vc {
        true => vc_gold_min,
        false => pbp_gold_min
    }[diff.get_index()] as f32 * time * _tier;
    let gold_max = match vc {
        true => vc_gold_max,
        false => pbp_gold_max
    }[diff.get_index()] as f32 * time * _tier;
    let downtime = (dt_mult[diff.get_index()] * time / 2.0).round() as i32;
    let gold = rand::thread_rng().gen_range(gold_min..gold_max).round() as i32;
    let vc_str = match vc {
        true => "VC",
        false => "PBP",
    };

    let reply = format!("The rewards for your tier {}, {} hour, {} {} quest\
    \nStamps: {}\nGold: {}\nDT: {}", _tier, time, diff.get_text(), vc_str, stamps, gold, downtime );

    ctx.say(reply).await?;
    Ok(())
}

async fn right_channel(ctx: Context<'_>) -> Result<bool, Error> {
    let channel_id = ctx.channel_id().get().to_string();
    Ok(channel_id == get_config_val(SecretType::StaffBots))
}

fn get_overall_tier(t1: Option<u32>, t2: Option<u32>, t3: Option<u32>, t4: Option<u32>) -> f32 {
    let tt1 = min(t1.unwrap_or(0), 1) as f32;
    let tt2 = min(t2.unwrap_or(0), 1) as f32;
    let tt3 = min(t3.unwrap_or(0), 1) as f32;
    let tt4 = min(t4.unwrap_or(0), 1) as f32;
    (1.0*tt1 + 2.0*tt2 + 3.0*tt3 + 4.0*tt4) / (tt1 + tt2 + tt3 + tt4)
}