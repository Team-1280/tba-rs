use reqwest::Url;
use serde::Deserialize;
use async_trait::async_trait;
use crate::ctx::endpoints::EndPoint;

use super::{id::{TeamNumber, Key, KeyReferenced}, Year};

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
    pub simple: SimpleTeam,
    pub address: Option<String>,
    pub postal_code: Option<String>,
    pub gmaps_place_id: Option<String>,
    pub gmaps_url: Option<Url>,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub location_name: Option<String>,
    pub website: Option<Url>,
    pub rookie_year: Option<Year>,
    pub motto: Option<String>,
    pub home_championship: HomeChampionshipsList,
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

#[async_trait]
impl KeyReferenced for Team {
    async fn dereference(key: Key<Self>, ctx: &crate::ctx::Context) -> Result<Self, crate::Error> {
        ctx
            .endpoints
            .team 
            .team
            .get((key,), ctx)
            .await
    } 
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
