use shiplift::{ContainerOptions, Docker};

#[test]
fn start_container() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    async fn run() {
        let docker = Docker::new();

        let options = ContainerOptions::builder("softwaremill/elasticmq:0.15.7")
            .name("raiden-sqs-elasticmq")
            .expose(9324, "tcp", 9324)
            .auto_remove(true)
            .build();

        docker
            .containers()
            .create(&options)
            .await
            .expect("should create");

        docker
            .containers()
            .get("raiden-sqs-elasticmq")
            .start()
            .await
            .expect("should start");
    }
    rt.block_on(run());
}
