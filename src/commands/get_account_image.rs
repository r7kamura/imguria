use crate::client::Client;
use crate::opt::Opt;
use crate::result::Result;

pub async fn get_account_image(opt: Opt) -> Result<()> {
    if let Opt::GetAccountImage {
        access_token,
        client_id,
        image_id,
        user_name,
    } = opt
    {
        let client = Client::builder()
            .credentials(access_token, client_id)
            .build()?;
        let basic = client.get_account_image(user_name, image_id).send().await?;
        dbg!(basic);
        Ok(())
    } else {
        panic!()
    }
}
