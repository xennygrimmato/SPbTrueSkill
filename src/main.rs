mod read_codeforces;
mod rating_system;

use rating_system::{Contest, simulate_contest};
use read_codeforces::{get_contest, get_contest_ids};
use read_codeforces::Contest as EbTechContest;
use std::time;


fn contest_adaptor(from: &EbTechContest) -> (Contest, usize) {
    let mut ans = Contest::new();

    for i in 1..from.standings.len() {
        assert!(from.standings[i - 1].1 <= from.standings[i].1);
    }

    let mut prev = usize::MAX;

    for (user, lo, _hi) in &from.standings {
        if *lo != prev {
            ans.push(Vec::new());
        }
        ans.last_mut().unwrap().push(vec![user.clone()]);

        prev = *lo;
    }

    (ans, from.time_seconds)
}


fn main() {
    let mut rating = rating_system::RatingHistory::new();

    let now = time::Instant::now();

    for contest_id in get_contest_ids() {
        let contest: EbTechContest = get_contest("cache", contest_id);
        println!(
            "Processing {:5} contestants in contest/{:4}: {}",
            contest.standings.len(),
            contest.id,
            contest.name
        );
        let adapted = contest_adaptor(&contest);
        simulate_contest(&mut rating, &adapted.0, adapted.1);
    }

    use std::io::Write;
    let filename = "data/CFratings.txt";
    let file = std::fs::File::create(filename).expect("Output file not found");
    let mut out = std::io::BufWriter::new(file);
    let mut to_sort = Vec::new();

    for (key, value) in &rating {
        to_sort.push((key.clone(), value.clone()));
    }

    to_sort.sort_by(|(_ak, av), (_bk, bv)|
        av.last().unwrap().0.mu.partial_cmp(&bv.last().unwrap().0.mu).unwrap());
    to_sort.reverse();

    let mut ord = 1;
    for (key, value) in to_sort {
        write!(out, "{}.\t{:30}", ord, key).ok();
        ord += 1;
        for (rating, _when) in &value[value.len() - 1..value.len()] {
            write!(out, "\t({:.2}, {:.2})", rating.mu, rating.sigma).ok();
        }
        writeln!(out).ok();
    }

    println!("Finished in {:.2} seconds", now.elapsed().as_secs_f64());
}
