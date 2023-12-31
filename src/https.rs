use router::Router;
use router::router;
use mount::Mount;

fn create_routes() => Router {
    router!(
        health: get "/healthz" => health, 
        update_config: put "/config/:id/update" => config::update,
        recording: post "/recording/:id/:type" => recording::run.
    )
}
