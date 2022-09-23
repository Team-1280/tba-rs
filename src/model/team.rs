use reqwest::Url;
use serde::Deserialize;

use super::id::TeamNumber;


#[derive(Clone, Deserialize)]
pub struct SimpleTeam {
    pub key: String,
    pub team_number: TeamNumber,
    pub nickame: Option<String>,
    pub name: String,
    pub city: Option<String>,
    pub state_prov: Option<String>,
    pub country: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct Team {
    simple: SimpleTeam,
    address: Option<String>,
    postal_code: Option<String>,
    gmaps_place_id: Option<String>,
    gmaps_url: Option<Url>,
    lat: Option<f64>,
    lng: Option<f64>,
    location_name: Option<String>,
    website: Option<Url>,
    rookie_year: Option<u16>,
    motto: Option<String>,
    home_championship: HomeChampionshipsList,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HomeChampionshipsList(Vec<(u16, String)>);

impl AsRef<SimpleTeam> for Team {
    fn as_ref(&self) -> &SimpleTeam {
        &self.simple
    }
}
impl AsMut<SimpleTeam> for Team {
    fn as_mut(&mut self) -> &mut SimpleTeam {
        &mut self.simple
    }
}
