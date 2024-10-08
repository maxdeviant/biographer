use chrono::prelude::*;
use console::{style, Term};
use dialoguer::Confirmation;

struct Bio {
    pub description: String,
}

struct Service {
    pub name: String,
    pub update_profile_url: String,
}

fn main() -> Result<(), &'static str> {
    femme::start(log::LevelFilter::Trace).unwrap();

    let birth_date = Local.with_ymd_and_hms(1994, 8, 24, 0, 0, 0).unwrap();
    let today = Local::now();
    let age = today.years_since(birth_date).unwrap_or(0);

    let bio = Bio {
        description:
            format!("宝霞龙. {age}. Conjurer of code. Devourer of art. Pursuer of æsthetics. Soli Deo gloria. He/him."),
    };

    let services = vec![
        Service {
            name: "are.na".into(),
            update_profile_url: "https://www.are.na/settings".into(),
        },
        Service {
            name: "bluesky".into(),
            update_profile_url: "https://bsky.app/profile/maxdeviant.com".into(),
        },
        Service {
            name: "twitter".into(),
            update_profile_url: "https://twitter.com/settings/profile".into(),
        },
        Service {
            name: "merveilles.town".into(),
            update_profile_url: "https://merveilles.town/settings/profile".into(),
        },
        Service {
            name: "github".into(),
            update_profile_url: "https://github.com/settings/profile".into(),
        },
        Service {
            name: "lobsters".into(),
            update_profile_url: "https://lobste.rs/settings".into(),
        },
    ];

    let term = Term::stdout();

    for service in services {
        loop {
            term.write_line(&format!("[++ {} ++]", style(&service.name).cyan()))
                .unwrap();
            term.write_str("  ").unwrap();
            term.write_line("Bio:").unwrap();
            term.write_str("    ").unwrap();
            term.write_line(&bio.description).unwrap();
            term.write_line("").unwrap();
            term.write_str("  ").unwrap();
            term.write_line(&format!(
                "Open {}",
                style(&service.update_profile_url).underlined()
            ))
            .unwrap();
            term.write_line("").unwrap();

            let done = Confirmation::new()
                .with_text(&format!("Are you finished updating {}?", service.name))
                .interact()
                .unwrap();
            if done {
                break;
            }
        }
    }

    println!("All your bios have been updated!");

    Ok(())
}
