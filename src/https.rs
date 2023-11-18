fn url() -> String {
   "https://httpbin.org/drip?duration=5&numbytes=13".into()
}

pub async fn get() -> std::result::Result<String, String> {
   match reqwest::get(url()).await {
      Ok(res) => match res.bytes().await {
         Ok(bytes) => Ok(format!("{:02X}", bytes)),
         Err(e) => Err(e.to_string()),
      },
      Err(e) => Err(e.to_string()),
   }
}
