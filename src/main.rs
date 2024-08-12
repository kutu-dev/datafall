use std::thread;

use datafall;

fn main() -> Result<(), String> {
    let mut threads = Vec::new();

    let download_urls = vec![
        "https://myrient.erista.me/files/No-Intro/ACT%20-%20Apricot%20PC%20Xi/%5BBIOS%5D%20MS-DOS%202.11%20%28Europe%29%20%28v2.7%29%20%28Disk%201%29%20%28OS%29.zip",
        "https://myrient.erista.me/files/No-Intro/ACT%20-%20Apricot%20PC%20Xi/%5BBIOS%5D%20MS-DOS%202.11%20%28Europe%29%20%28v2.7%29%20%28Disk%202%29%20%28OS%29.zip",
        "https://myrient.erista.me/files/No-Intro/ACT%20-%20Apricot%20PC%20Xi/%5BBIOS%5D%20MS-DOS%202.11%20%28Europe%29%20%28v2.7%29%20%28Disk%203%29%20%28OS%29.zip",
    ];

    for url in download_urls {
        let thread = thread::spawn(move || {
            datafall::download_file(String::from(url));
        });

        threads.push(thread);
    }

    for thread in threads {
        thread.join();
    }

    Ok(())
}
