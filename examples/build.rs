fn main() {
    tonic_build::configure()
        .type_attribute("routeguide.Point", "#[derive(Hash)]")
        .compile(&["proto/routeguide/route_guide.proto"], &["proto"])
        .unwrap();

    tonic_build::compile_protos("proto/helloworld/helloworld.proto").unwrap();
    tonic_build::compile_protos("proto/echo/echo.proto").unwrap();

    tonic_build::configure()
        .build_server(false)
        .out_dir("src/gcp/google") // comment this line to generate code to the default location
        .compile(
            &["proto/googleapis/google/pubsub/v1/pubsub.proto"],
            &["proto/googleapis"],
        )
        .unwrap();
}
