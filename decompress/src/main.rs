use std::{env::args, ffi::OsStr, fs, io, path, process};
use zip::read::ZipArchive;
fn main() {
    process::exit(real_main())
}

fn real_main() -> i32 {
    let args: Vec<_> = args().skip(1).collect();
    if args.len() < 1 {
        eprintln!("参数错误");
        return 1;
    }
    let fname = path::Path::new(&*args[0]);
    if fname.is_dir() {
        eprintln!("这不是一个文件路径:{:?}", fname.file_name().unwrap());
        return 1;
    }
    if fname.is_file() {
        let is_zip = match fname.extension().and_then(OsStr::to_str) {
            Some("zip") => true,
            _ => false,
        };
        if !is_zip {
            eprintln!("不是一个zip文件");
            return 1;
        }
    }
    let file = fs::File::open(fname).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        println!("文件名: {:?}, 大小: {}", &file.name(), &file.size());

        let output_path = match file.enclosed_name() {
            Some(r) => r.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("file {} comment {}", i, comment);
            }
        }

        if (*file.name()).ends_with("/") {
            println!("file {} extracted to \"{}\"", i, output_path.display());
            fs::create_dir_all(&output_path).unwrap();
        } else {
            println!(
                "file {} extracted to \"{}\" ({} bytes)",
                i,
                output_path.display(),
                file.size()
            );

            if let Some(p) = output_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut out_file = fs::File::create(&output_path).unwrap();
            io::copy(&mut file, &mut out_file).unwrap();
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&output_path, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    0
}
