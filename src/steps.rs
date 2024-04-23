use cucumber::{given, when, then};

#[given(regex = r"^I need to sleep for (\d+) second(s)?$")]
pub async fn setup_sleep(world: &mut crate::World, secs: u64) {
    world.sleeper = secs;
}

#[when(regex = r"^I sleep$")]
pub async fn sleep(world: &mut crate::World) {
    let dur = tokio::time::Duration::from_millis(world.sleeper * 1000);
    tokio::time::sleep(dur).await;
    
    world.refreshed = true;
}

#[then(regex = r"^I feel refreshed$")]
pub async fn im_refreshed(world: &mut crate::World) {
    assert!(world.refreshed);
}
