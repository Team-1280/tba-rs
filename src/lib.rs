pub mod model;
pub mod ctx;
pub mod error;

pub use error::Error;

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use crate::{model::{Year, team::Team, id::KeyReferenced}, ctx::{Context, endpoints::EndPoint}};

    use super::*;

    #[tokio::test]
    async fn event_test() {
        let ctx = Context::authenticate(
            std::fs::read_to_string("token.txt")
                .unwrap()
        ).unwrap_or_else(|e| panic!("{}", e));
        let year = Year::new(2013).unwrap(); 
        let teams = ctx
            .endpoints
            .teams
            .keys_by_year
            .get((year,1), &ctx)
            .await
            .unwrap();
        for key in teams.iter().take(5) {
            let team = Team::dereference(key.clone(), &ctx).await.unwrap();
            let team2 = Team::dereference(key.clone(), &ctx).await.unwrap();
            assert!(Arc::ptr_eq(&team, &team2));
        }
    }
}
