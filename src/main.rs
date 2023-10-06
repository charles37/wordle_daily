use axum::{http::StatusCode, response::IntoResponse, Router, routing::get};
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use http::uri::Uri;

async fn redirect() -> impl IntoResponse {
    let wordle_variants = vec![
        "https://www.nytimes.com/games/wordle/index.html",
        "https://www.devangthakkar.com/wordle_archive/?222",
        "https://hellowordl.net/",
        "https://www.wordleunlimited.com/",
        "https://engaging-data.com/wordguessr-wordle/",
        "http://wordle.jonyork.net/",
        "https://squabble.me/",
        "https://mottaquikarim.github.io/wordle_with_friends/",
        "https://wordhoot.com/",
        "https://zaratustra.itch.io/dordle",
        "https://engaging-data.com/tridle/",
        "https://www.quordle.com/",
        "https://octordle.com/",
        "http://sedecordle.com/?mode=daily",
        "https://jonesnxt.github.io/kilordle/",
        "https://polydle.github.io/",
        "https://doodle-pile.gitlab.io/thirtle/",
        "https://crosswordle.serializer.ca/",
        "https://fubargames.se/squardle/",
        "https://qntm.org/files/absurdle/absurdle.html",
        "https://swag.github.io/evil-wordle/",
        "https://limpet.net/qwrtl/",
        "https://wafflegame.net/",
        "http://reversle.net",
        "https://lazyguyy.github.io/survivle/",
        "https://www.timblack.net/absurvivle/",
        "https://engaging-data.com/scrabwordle/",
        "https://xordle.xyz/",
        "https://fibble.xyz/",
        "https://word.rodeo/Sixdle/",
        "https://warmle.org/",
        "https://vegeta897.github.io/wordle-peaks/",
        "https://www.symble.app/",
        "https://wordle.louan.me/",
        "https://sutom.nocle.fr/",
        "https://sebastianomorando.github.io/wordle-it/",
        "https://wordle.danielfrg.com/",
        "https://term.ooo/",
        "https://ordlig.se/",
        "https://wordle-spielen.de/",
        "https://wordle.uber.space/",
        "https://wordle.uber.space/",
        "https://meduyeket.net/",
        "https://kerdle.vercel.app/",
        "https://nakosung.github.io/wordle/",
        "https://www.rhwyd.org/wordle/",
        "https://tilde.town/~dustin/wordle-toki/",
        "https://manishearth.github.io/ipadle/#",
        "https://wordawazzle.com.au/",
        "https://converged.yt/primel/",
        "https://www.mathler.com/",
        "https://nerdlegame.com/",
        "https://www.taylordle.com/",
        "https://rbrignall.github.io/byrdle/",
        "https://agreenerworldle.org/",
        "https://bts-wordle.vercel.app/",
        "https://www.moxfield.com/moxle",
        "https://sweardle.com/",
        "https://www.jellyneo.net/jordle/",
        "https://queerdle.com/",
        "https://ygo-wordle.vercel.app/",
        "https://oundle.andrewchapman.info/",
        "https://www.lewdlegame.com/",
        "https://paimordle.vercel.app/",
        "https://brdl.alex.gd/",
        "https://www.nytimes.com/games/wordle/index.html",
        "https://commandercodex.com/",
        "https://squirdle.fireblend.com/",
        "https://digitaltolkien.github.io/vue-wordle/",
        "https://wordle.starwars.guide/",
        "https://airportle.glitch.me/",
        "https://airportle.scottscheapflights.com/",
        "https://www.subwaydle.com/",
        "https://worldle.teuteuf.fr/",
        "https://globle-game.com/",
        "https://oec.world/en/tradle/",
        "https://www.flagdle.org/",
        "https://www.flagle.io/",
        "https://edjefferson.com/letterle/",
        "https://www.dialup.net/windle/",
        "https://rsk0315.github.io/playground/passwordle.html",
        "https://semantle.novalis.org/",
        "http://scidle.co.uk/",
        "https://jackli.gg/chessle/",
        "https://yewang.github.io/fusekle/",
        "https://pictle.web.app/",
        "https://www.heardle.app/",
        "https://squaredle.app/",
    ];

    let start_of_unix_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let days_since_unix_epoch = start_of_unix_time.as_secs() / 86400;

    let index = (days_since_unix_epoch % wordle_variants.len() as u64) as usize;

    let redirect_uri = Uri::from_static(wordle_variants[index]);
    (StatusCode::FOUND, axum::response::Redirect::temporary(redirect_uri))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(redirect));
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
