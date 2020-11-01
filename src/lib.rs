use std::collections::HashSet;
use std::thread::spawn;
use zip::ZipArchive;
use std::io::{Cursor, Read};

pub fn get_sample_name_bytes(distribution_size: usize) -> Vec<(String, Vec<u8>)> {
    // See NOTICE
    let urls = [
        "https://repo1.maven.org/maven2/com/google/guava/guava/30.0-jre/guava-30.0-jre.jar",
        "https://repo1.maven.org/maven2/com/squareup/okhttp3/okhttp/4.10.0-RC1/okhttp-4.10.0-RC1.jar",
        "https://repo1.maven.org/maven2/org/apache/spark/spark-core_2.11/2.4.7/spark-core_2.11-2.4.7.jar",
        "https://repo1.maven.org/maven2/com/google/zxing/core/3.4.1/core-3.4.1.jar",
        "https://repo1.maven.org/maven2/com/google/inject/guice/5.0.0-BETA-1/guice-5.0.0-BETA-1.jar",
        "https://repo1.maven.org/maven2/junit/junit/4.13.1/junit-4.13.1.jar",
        "https://repo1.maven.org/maven2/org/jetbrains/kotlin/kotlin-compiler/1.4.20-M1/kotlin-compiler-1.4.20-M1.jar",
        "https://repo1.maven.org/maven2/org/scala-lang/scala-compiler/2.13.3/scala-compiler-2.13.3.jar",
        "https://repo1.maven.org/maven2/org/bitcoinj/bitcoinj-core/0.15.8/bitcoinj-core-0.15.8.jar"
    ];
    let mut sizes = HashSet::new();
    urls.iter().map(|&url| {
        let thing = spawn(move || {
            let mut easy = curl::easy::Easy::new();
            let mut dst = vec![];
            easy.url(url).unwrap();
            let mut transfer = easy.transfer();
            transfer.write_function(|data| {
                dst.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();
            transfer.perform().unwrap();
            drop(transfer);
            dst
        });
        ZipArchive::new(Cursor::new(thing.join().unwrap())).unwrap()
    }).flat_map(move |mut zip| {
        let len = zip.len();
        let mut vec_classes = vec![];
        for i in 0..len {
            let mut zipfile = zip.by_index(i).unwrap();
            let name = zipfile.name().to_owned();
            let size = zipfile.size() / distribution_size;
            if zipfile.is_file() && name.ends_with(".class") && !sizes.contains(&size) {
                sizes.insert(size);
                let mut vec = vec![];
                zipfile.read_to_end(&mut vec).unwrap();
                vec_classes.push((zipfile.name().to_string(), vec))
            }
        }
        vec_classes
    }).collect()
}