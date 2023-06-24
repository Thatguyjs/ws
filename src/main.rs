/*
   * STRUCTURE *
    
    main
        multiple host locations
        safely stopping server
        server config
            config file
            CLI options
        mime
        response codes
        http
            buffer
            safely shutting down connections
            file system access
            file streaming
            https
                server certs
            error pages
            headers
            body
            request obj
                streaming requests
            response obj
                streaming responses
        threads

   * GROUPS *
    
    [Remote host] -> Connection -> Accept -> [Create] Thread ->
        [Create] Adapter

    TcpListener -> [std] TcpListener + TcpShutdown

    HttpRequest -> Adapter -> [Request Data]
    HttpResponse -> Adapter -> [Response Data]
*/

mod http;
mod listener;
mod threadpool;


fn main() {
    // Parse config & start server, provide Ctrl-C handling
    let (server, mut shutdown) = http::HttpServer::bind("127.0.0.1:8080".parse().unwrap(), None).unwrap();

    ctrlc::set_handler(move || {
        shutdown.shutdown().unwrap();
    }).expect("Failed to set Ctrl-C handler");

    server.run().expect("Failed to start server");
}
