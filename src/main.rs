
use aws_sdk_s3::Client;
use aws_sdk_s3::Region;
use aws_sdk_s3::Endpoint;
use aws_types::credentials::SharedCredentialsProvider;
use aws_types::credentials::Credentials;
use std::error::Error;
use std::io::{self, Write};
use aws_smithy_http::byte_stream::ByteStream;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Configure a credencial
    let credentials = Credentials::new("dummy-access-key", "dummy-secret-key", None, None, "dummy");

    // Configure o cliente do S3 apontando para o LocalStack
    let config = aws_sdk_s3::Config::builder()
        .region(Region::new("us-east-1"))
        .endpoint_resolver(Endpoint::immutable("http://localhost:4566".parse()?))
        .credentials_provider(SharedCredentialsProvider::new(credentials))
        .build();
    
    let client = Client::from_conf(config);

    // Nome do bucket e arquivo
    let bucket_name = "meu-bucket1";
    let file_name = "exemplo.txt";
    let file_contents = String::from("Este é um arquivo de exemplo.");

    // Crie um bucket
    client.create_bucket().bucket(bucket_name).send().await?;
    println!("Bucket '{}' criado.", bucket_name);

    // Faça upload de um arquivo no bucket
    client.put_object()
        .bucket(bucket_name)
        .key(file_name)
        .body(file_contents.into_bytes().into())
        .send()
        .await?;
    println!("Arquivo '{}' carregado para o bucket '{}'.", file_name, bucket_name);

    print!("Por favor, digite d para donwload: ");
    // Necessário para garantir que a mensagem seja exibida antes de esperar a entrada
    io::stdout().flush().unwrap();

    // Criar uma nova String para armazenar a entrada do usuário
    let mut input = String::new();

    // Ler a entrada do usuário
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");

    // Remover qualquer espaço em branco extra (como newline) do fim da String
    let input = input.trim();

    if input == "d"{
        println!("d digitado");

        let get_object_output= client.get_object()
        .bucket(bucket_name)
        .key(file_name)
        .send()
        .await?;

        // Convert the ByteStream to a bytes Vector (Vec<u8>)
        // let bytes = byte_stream.collect().await?.into_bytes();
        let collected = get_object_output.body.collect().await.unwrap().into_bytes();
        
        let str = String::from_utf8 (collected.to_vec()).unwrap();
        println!("{}", str);

    }
    Ok(())
}