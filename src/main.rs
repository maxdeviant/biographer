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

    let bio = Bio {
        description:
            "宝霞龙. 25. Conjurer of code. Devourer of art. Pursuer of æsthetics. Soli Deo gloria."
                .into(),
    };

    let services = vec![
        Service {
            name: "are.na".into(),
            update_profile_url: "https://www.are.na/settings/general".into(),
        },
        Service {
            name: "twitter".into(),
            update_profile_url: "https://twitter.com/settings/profile".into(),
        },
        Service {
            name: "merveilles.town".into(),
            update_profile_url: "https://merveilles.town/settings/profile".into(),
        },
    ];

    for service in services {
        loop {
            println!("{}", service.name);
            println!("Open {}", service.update_profile_url);
            println!("{}", bio.description);

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
