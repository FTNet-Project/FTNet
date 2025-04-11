use crate::ACK;
use eyre::WrapErr;

pub fn id52_to_public_key(id: &str) -> eyre::Result<iroh::PublicKey> {
    let bytes = data_encoding::BASE32_DNSSEC.decode(id.as_bytes())?;
    if bytes.len() != 32 {
        return Err(eyre::anyhow!(
            "read: id has invalid length: {}",
            bytes.len()
        ));
    }

    let bytes: [u8; 32] = bytes.try_into().unwrap(); // unwrap ok as already asserted

    iroh::PublicKey::from_bytes(&bytes).wrap_err_with(|| "failed to parse id to public key")
}

pub fn public_key_to_id52(key: &iroh::PublicKey) -> String {
    data_encoding::BASE32_DNSSEC.encode(key.as_bytes())
}

pub type FrameReader =
    tokio_util::codec::FramedRead<iroh::endpoint::RecvStream, tokio_util::codec::LinesCodec>;

pub fn frame_reader(recv: iroh::endpoint::RecvStream) -> FrameReader {
    FrameReader::new(recv, tokio_util::codec::LinesCodec::new())
}

pub async fn get_remote_id52(conn: &iroh::endpoint::Connection) -> eyre::Result<String> {
    let remote_node_id = match conn.remote_node_id() {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("could not read remote node id: {e}, closing connection");
            // TODO: is this how we close the connection in error cases or do we send some error
            //       and wait for other side to close the connection?
            let e2 = conn.closed().await;
            tracing::info!("connection closed: {e2}");
            // TODO: send another error_code to indicate bad remote node id?
            conn.close(0u8.into(), &[]);
            return Err(eyre::anyhow!("could not read remote node id: {e}"));
        }
    };

    Ok(public_key_to_id52(&remote_node_id))
}

pub async fn ack(send: &mut iroh::endpoint::SendStream) -> eyre::Result<()> {
    send.write_all(format!("{}\n", ACK).as_bytes()).await?;
    Ok(())
}
