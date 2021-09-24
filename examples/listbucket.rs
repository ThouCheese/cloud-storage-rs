use std::borrow::BorrowMut;

use cloud_storage::{Client, ListRequest};
use futures::TryStreamExt;

type AnyResult<T> = anyhow::Result<T>;

#[tokio::main]
async fn main() -> AnyResult<()> {
    let client = Client::default();
    let mut rq = ListRequest::default();
    rq.prefix = Some("<ABigPrefixWithMoreThan60KObjects>".to_owned());
    rq.fields = Some("items(selfLink),nextPageToken".to_owned());

    let r = client
        .object()
        .list("<YourBucket>", rq)
        .await?
        .try_fold(Vec::new(), |mut s, mut x| async move {
            s.append(x.items.borrow_mut());
            Ok(s)
        })
        .await?;

    println!("list {:?}", r.len());
    Ok(())
}
