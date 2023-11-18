use async_fs::{File, OpenOptions};

fn filepath() -> String {
   let path = "/data/user/0/com.example.etude_androidapp_rust";
   let filepath = format!("{path}/tmp.txt");
   return filepath;
}

fn ioerr_to_string(head: &str, e: &std::io::Error) -> String {
   format!(
      "{}: {}: {}",
      head,
      std::io::Error::from_raw_os_error(e.raw_os_error().unwrap_or(0)),
      filepath()
   )
}

pub async fn read_count() -> std::result::Result<u32, String> {
   let mut data = Vec::<u8>::new();
   {
      use futures_lite::io::AsyncReadExt;
      let opt_f = match File::open(&filepath()).await {
         Ok(f) => Ok(Some(f)),
         Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => Ok(None),
            _ => Err(ioerr_to_string("open to read", &e)),
         },
      }?;
      if opt_f.is_none() {
         return Ok(0);
      } else if let Some(mut f) = opt_f {
         f.read_to_end(&mut data)
            .await
            .map_err(|e| ioerr_to_string("read", &e))?;
      }
   }
   log::debug!("read {:?}", data);
   match String::from_utf8(data) {
      Ok(s) => s
         .parse()
         .map_err(|_| format!("parse fail: {}", s))
         .map(|v| {
            log::debug!("read={}", v);
            v
         }),
      Err(_) => Err("from_utf8".to_string()),
   }
}

pub async fn write_count(count: u32) -> Result<u32, String> {
   use futures_lite::io::AsyncWriteExt;
   let mut file = OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(filepath())
      .await
      .map_err(|e| ioerr_to_string("open to write", &e))?;
   log::debug!("write {}", count);
   file
      .write_all(format!("{count}").as_str().as_bytes())
      .await
      .map_err(|e| ioerr_to_string("write", &e))?;
   file
      .sync_all()
      .await
      .map_err(|e| ioerr_to_string("sync", &e))?;
   log::debug!("read {:?}", read_count().await);
   Ok(count)
}
