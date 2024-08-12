use datafall;

use relm4::prelude::*;
use relm4::gtk::prelude::*;

struct App {
}

#[relm4::component]
impl SimpleComponent for App {
    type Input = ();
    type Output = ();
    type Init = ();
    
    view! {
        gtk::Window {
            
        }
    }

    fn init(
        counter: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = App{};
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
    }
}

#[tokio::main]
async fn main() {
    let app = RelmApp::new("dev.dobon.datafall");
    app.run::<App>(());
}











async fn stale() -> Result<(), String> {
    let mut tasks = Vec::new();

    let download_urls = vec![
        "https://myrient.erista.me/files/Redump/Nintendo%20-%20Wii%20-%20NKit%20RVZ%20%5Bzstd-19-128k%5D/Super%20Mario%20Galaxy%20%28Europe%2C%20Australia%29%20%28En%2CFr%2CDe%2CEs%2CIt%29.zip",
        "https://myrient.erista.me/files/Redump/Nintendo%20-%20Wii%20-%20NKit%20RVZ%20%5Bzstd-19-128k%5D/Super%20Mario%20Galaxy%202%20%28Europe%29%20%28En%2CFr%2CDe%2CEs%2CIt%29.zip",
    ];

    for url in download_urls {
        let task = tokio::spawn(async {
            let result = datafall::download_file(url).await;
            
            if let Err(error) = result {
                println!("{error}");
            }
        });

        tasks.push(task);
    }

    for task in tasks {
        task
        .await
        .map_err(|error| format!("A file download task has failed: {error}"))?;
    }

    Ok(())
}
