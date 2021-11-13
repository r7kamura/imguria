use crate::client::Client;
use crate::opt::Opt;
use crate::result::Result;

pub async fn get_image(opt: Opt) -> Result<()> {
    if let Opt::GetImage {
        access_token,
        client_id,
        hash,
    } = opt
    {
        let client = Client::builder()
            .credentials(access_token, client_id)
            .build()?;
        let basic = client.get_image(hash).send().await?;
        dbg!(basic);
        Ok(())
    } else {
        panic!()
    }
}