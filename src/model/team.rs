use reqwest::Url;
use serde::Deserialize;

use super::{id::{TeamNumber, Key}, Year};

pub type TeamKey = Key<Team>;
pub type RobotKey = Key<TeamRobot>;

/// Structure representing basic data about an FRC team, that can be upgraded using a
/// [Context](crate::ctx::Context)
#[derive(Clone, Deserialize, Debug)]
pub struct SimpleTeam {
    pub key: TeamKey,
    pub team_number: TeamNumber,
    pub nickame: Option<String>,
    pub name: String,
    pub city: Option<String>,
    pub state_prov: Option<String>,
    pub country: Option<String>,
}

/// A team object containing more data than a [SimpleTeam]
#[derive(Clone, Deserialize, Debug)]
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
    rookie_year: Option<Year>,
    motto: Option<String>,
    home_championship: HomeChampionshipsList,
}


/// A newtype containing a map of year numbers to the location of a home championship
#[derive(Clone, Debug,)]
pub struct HomeChampionshipsList(
    Vec<(u16, String)>
);

/// A robot that competed in a given [Year] with name and a [TeamKey] referencing the team that
/// created this robot
#[derive(Clone, Debug, Deserialize)]
pub struct TeamRobot {
    pub year: Year,
    pub robot_name: String,
    pub key: RobotKey,
    pub team_key: TeamKey,
}

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

impl<'de> Deserialize<'de> for HomeChampionshipsList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        use serde::de::Visitor;
        struct MapVisitor;
        impl<'de> Visitor<'de> for MapVisitor {
            type Value = HomeChampionshipsList;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "Map of year numbers to home championship locations")    
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>, {
                let mut list = Vec::with_capacity(map.size_hint().unwrap_or(5));
                while let Some((k, v)) = map.next_entry()? {
                    list.push((k, v))
                }

                Ok(HomeChampionshipsList(list))
            }
        }

        deserializer.deserialize_map(MapVisitor)
    }
}
